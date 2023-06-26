use crate::model::encode_text;
use hora::core::ann_index::ANNIndex;
use rayon::prelude::*;
use hora::index::hnsw_idx::HNSWIndex;
use rusqlite::{Connection, Statement};

use crate::document::{
    Document,
    get_file_list,
    get_file_text,
    document_uploads
};

use crate::vectordb::{
    create_sqlite_db,
    create_index,
    load_index, load_sqlite_db
};


use std::fs::create_dir;
use std::path::PathBuf;
use std::io::ErrorKind;
use std::sync::Arc;


#[derive(Debug)]
pub struct Matches{
    pub document_name:String,
    pub line:String,
}

#[derive(Debug)]
pub struct Cephalon{
    path:PathBuf,
    documents:Option<Vec<Document>>
}

impl Cephalon{

    fn get_text_from_all_docs(self, doc_list:&mut Vec<Document>){
        doc_list.par_iter_mut().for_each(|doc: &mut Document|{
            println!("Now Processing {:?} ...",doc.get_document_name_as_string());
            let document_data_option: Option<Vec<String>> = get_file_text(doc,256);
            match document_data_option{
                Some(doc_text)=> doc.set_document_data(doc_text),
                None=>println!("Error reading document {:?}",doc.get_document_name_as_string())
            }
            println!("\r Finished Processing file {:?}",doc.get_document_name_as_string());
        });
    }
    
    
    ///Description: This function will the current directory for all files, and store all the supported file_types as
    ///a Vector of Documents doc_list. This is done by calling the get_file_list function. Then using doc_list extract all text from the supported file types and store the data
    ///in the data attribute of Document Structure respectively.That is done by calling the get_text_from_all_docs function. 
    ///Next it will split the text for each document into an array of string. Each string will be the size of tokens that could be accepted by 
    ///an embedding model in sentence_transformer. 
    pub fn search_and_build_index(self, path:&PathBuf){
        let mut project_path: PathBuf = path.clone();
        project_path.push(".cephalon");
        //Get all the supported Documents from the directory and store it in documents
        let mut doc_list:Vec<Document>;
        match get_file_list(path){
            Ok(f_list)=>{
                doc_list=f_list;
            },
            Err(err)=>panic!("{:?}",err)
        }

        //Extract text from all the documents in the doc_list
        self.get_text_from_all_docs(&mut doc_list);

        //Generate encodings for all the text of a document in the list doc_list
        Document::encode_and_upload_documents(&mut doc_list, (*project_path).to_path_buf());

    }

    ///Search Index for related queries, and covert it back to original text
    pub fn search(self, path:PathBuf, query:String,count:usize)->Option<Vec<Matches>>{
        let results:Vec<usize>;
        let mut project_path = path.clone();
        project_path.push(".cephalon");
        match encode_text(&vec![query]){ //Generate Embeddings for the query
            Some(encodings)=>{ 
                
                let index: HNSWIndex<f32, usize> = load_index(project_path.clone());
                results = index.search(&encodings[0], count);
            },
            None=>{
                return None
            }
        }
        let conn:Connection;
        match load_sqlite_db(&project_path){
            Some(sql_db)=>{
                conn=sql_db;
            },
            None=>{
                panic!("Error loading sql db to match searched.");
            }
        }
        
        let mut stmt: Statement<'_> = conn.prepare("SELECT DocumentName, Line FROM  Vectors WHERE Id = (?1)").unwrap();

        let mut search_results:Vec<Matches>=vec![];
        
        for result in results{
            let match_iter = stmt.query_map(&[&result], |row| {
                
                Ok(Matches {
                    document_name: row.get(0)?,
                    line: row.get(1)?,
                })
            }).unwrap();

            for m in match_iter{
                match m{
                    Ok(search_result)=>{
                        search_results.push(search_result);
                    },
                    Err(err)=>{
                        println!("Error getting search result: {:?}",err);
                    }
                }
            }
        }

        Some(search_results)

    }
    
}

pub trait Util{
    fn new(path:PathBuf)->Self;
    fn load(path:PathBuf)->Self;
}

impl Util for Cephalon{
    /// Create a new Cephalon struct
    fn new(path:PathBuf)->Cephalon{
        let mut project_path: PathBuf = path.clone();
        project_path.push(".cephalon");
        match create_dir(&project_path){
            Ok(_msg)=>println!("Created project folder"),
            Err(err)=> {
                if err.kind() == ErrorKind::AlreadyExists{
                    println!("Loading Cephalon from previous project")
                }else{
                    panic!("Error creating cephalon project: {:?}",err)
                }
            }
        }




        //Create the index to be saved in .cephalon
        let _index: HNSWIndex<f32, usize> = create_index((*project_path).to_path_buf(),384);
        
        //Create the sqlite database to be saved in .cephalon
        let conn = create_sqlite_db((*project_path).to_path_buf());
        match conn.close(){
            Ok(_c)=>println!("Successfully created database"),
            Err(err)=>panic!("Error close database connection: {:?}",err)
        }

        Cephalon{path:path.to_path_buf(), documents:None}
    }

    /// Load an existing Cephalon project and return it as a struct. 
    fn load(path:PathBuf)->Cephalon{
        Cephalon{path:path.to_path_buf(), documents:None}
    }
}