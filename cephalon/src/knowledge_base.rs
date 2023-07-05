#[cfg(not(feature="no-ml"))]
use crate::models::{
    model::encode_text,
    summarize_model::generate_summary,
};

use clap::builder::OsStr;
use rayon::{prelude::*, vec};


use hora::index::hnsw_idx::HNSWIndex;
use hora::core::ann_index::{
    ANNIndex,
    SerializableIndex
};
use hora::core::metrics::Metric;

use crate::documents::document::{
    Document,
    get_file_text,
    get_file_list, is_supported, 
    UnsupportedDocumentError, DocType, get_text_from_docx, get_text_from_pdf, get_text_from_txt
};



use crate::database::vectordb::{
    create_index,
    load_index, 
    save_index,
};

use crate::database::sql_database::{
    create_sqlite_db,
    load_sqlite_db,
    insert_data_into_sql_db,
    sql_search_by_id
};


use std::fs::create_dir;
use std::path::PathBuf;
use std::io::ErrorKind;
use std::fmt;



type Result<T> = std::result::Result<T, KnowledgeBaseError>;

/// SQL Error
#[cfg(not(feature="no-ml"))]
#[derive(Debug, Clone)]
pub struct KnowledgeBaseError;


impl fmt::Display for KnowledgeBaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid sql transaction or connection")
    }
}

#[cfg(not(feature="no-ml"))]
#[derive(Debug)]
pub struct Matches{
    pub document_name:String,
    pub line:String,
}

#[cfg(not(feature="no-ml"))]
#[derive(Debug)]
pub struct Cephalon{
    path:PathBuf,
    documents:Option<Vec<Document>>
}

#[cfg(not(feature="no-ml"))]
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
            Some(f_list)=>{
                doc_list=f_list;
            },
            None=>{
                panic!("Unable to get a list of file!")
            }
        }

        //Extract text from all the documents in the doc_list
        self.get_text_from_all_docs(&mut doc_list);

        //Generate encodings for all the text of a document in the list doc_list
        Document::build_semantic_search(&mut doc_list, (*project_path).to_path_buf());

    }

    ///Search Index for related queries, and covert it back to original text
    pub fn search(self, path:PathBuf, query:String,count:usize)->Option<Vec<Matches>>{
        let mut results:Vec<usize> = vec![];
        let mut project_path = path.clone();
        project_path.push(".cephalon");
        match encode_text(&vec![query]){ //Generate Embeddings for the query
            Some(encodings)=>{
                for encoding in encodings{
                    let index: HNSWIndex<f32, usize> = load_index(project_path.clone());
                    match encoding.1{//Embeddings are stored in location 1 as Option<Vec<f32>>
                        Some(mut embedding)=>{
                            results.append(&mut index.search(&mut embedding, count));
                        },
                        None=>{}
                    }
                    
                }  
            },
            None=>{
                return None
            }
        }

        let mut search_results:Vec<Matches> = vec![];
        match sql_search_by_id(project_path,results){
            Some(search_output)=>{
                for output in search_output{
                    search_results.push(Matches{document_name:output.0, line:output.1});
                }
            },
            None=>{
                return None
            }
        }

        Some(search_results)

    }
    
}

#[cfg(not(feature="no-ml"))]
pub trait Util{
    fn new(path:PathBuf)->Self;
    fn load(path:PathBuf)->Self;
}

#[cfg(not(feature="no-ml"))]
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

#[cfg(not(feature="no-ml"))]
pub trait DocumentEncoder{
    fn build_semantic_search(doc_list:&mut Vec<Document>, project_path:PathBuf)->Result<()>;
    fn encode_text_via_model(&self, model:&str)->Option<Vec<(String,Option<Vec<f32>>)>>;
    fn load(file_path:String)->Result<Document>;
    fn summarize(&self)->Result<Vec<String>>;
}

#[cfg(not(feature="no-ml"))]
impl DocumentEncoder for Document{

    /// Building Semantic Search for a vector of documents. 
    fn build_semantic_search(doc_list:&mut Vec<Document>, project_path:PathBuf)->Result<()>{
        // We iterate through documents, generate the embeddings, and add the embeddings to index. 
        //Get the index
        let mut index:HNSWIndex<f32,usize> = create_index(project_path.clone(), 384);
        let mut id:usize = 0;

        for doc in doc_list{
            match doc.encode_text_via_model("all-MiniLM-L6-v2"){
                Some(vector_embeddings)=>{
                    let sentences:&Vec<String>; 
                    match doc.get_document_data(){
                        Some(data)=>{
                            sentences = data;
                            for embedding_data in vector_embeddings{
                                id+=1;
                                match embedding_data.1{
                                    Some(embedding)=>{
                                        //Insert it into the index
                                        match index.add(&embedding, id){
                                            Ok(_msg)=>{},
                                            Err(err)=>{
                                                println!("Error: {}, on id:{}",err,id);
                                            }
                                        }

                                        //Insert it into sql db for text retreival 
                                        match insert_data_into_sql_db(project_path.clone() ,&doc.get_document_name_as_string().unwrap(),&embedding_data.0,id){
                                            Ok(_msg)=>{},
                                            Err(err)=>{
                                                println!("Error inserting line:{} due to error:{:?}",embedding_data.0, err);
                                            }
                                }

                                    },
                                    None=>{

                                    }
                                }
                                
        
                                
                                
                            }
                        },
                        None=>{
                            println!("No Text found for file:{}",doc.get_document_name_as_string().unwrap());
                            continue
                        }
                    }

                },
                None=>{
                    println!("Error generating embeddings for: {:?}",doc.get_document_name_as_string().unwrap());
                    continue
                }
            }
        }

        save_index(&mut index, project_path);


        Ok(())
    }

    /// Encode text of the current document via a sentence embedding model.
    /// If the model was unable to encode the text into vector embeddings then 
    /// none is returned. 
    fn encode_text_via_model(&self, _model:&str)->Option<Vec<(String,Option<Vec<f32>>)>>{
        let mut encodings:Vec<Vec<f32>> = vec![];
        let sentences:Vec<String>;
        match self.get_document_data(){
            Some(vec_string)=>{
                sentences = vec_string.to_vec();
            },
            None=>{
                println!("Document has no parsed data");
                return None
            }
        }
        match encode_text(&sentences){
                Some(embedded_sentences)=>{
                    return Some(embedded_sentences)
                },
                None=>{
                    println!("Unable to generate Embeddings for document:{:?}",self.get_document_name_as_string());
                    return None
                }
        }
    }

    fn load(file_path:String)->Result<Document>{

        let file_metadata:std::fs::Metadata;

        match std::fs::metadata(&file_path){
            Ok(mdata)=>{
                file_metadata= mdata;
            },
            Err(err)=>{
                return Err(KnowledgeBaseError)
            }
        }

        if !file_metadata.is_file(){//If the Path is not a Document return an error
            return Err(KnowledgeBaseError)
        }
        let file_name:String;
        match std::path::Path::new(&file_path).file_name(){
            Some(name)=>{
                file_name = name.to_string_lossy().to_string();
            },
            None=>{
                return Err(KnowledgeBaseError)
            }
        }
        let document:Document = Document{// 
            name:file_name,
            path:PathBuf::from(file_path),
            metadata:file_metadata,
            data:None,
            encodings:None
        };
        Ok(document)
    }

    fn summarize(&self)->Result<Vec<String>> {
        let doc_text:String;
        
        let summary:Vec<String>;
        match self.get_document_data_as_string(){
            Ok(doc_text)=>{
                match generate_summary(doc_text){
                    Ok(doc_summary)=>{
                        summary = doc_summary;
                    },
                    Err(err)=>{
                        return Err(KnowledgeBaseError)
                    }
                }
            },
            Err(err)=>{
                return Err(KnowledgeBaseError)
            }
            
        }
        
        Ok(summary)
    }

}