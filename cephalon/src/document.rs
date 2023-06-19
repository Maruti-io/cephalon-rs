use minidom::{
    Element,
    Node,
    NSChoice, node
};

use hora::index::hnsw_idx::HNSWIndex;
use hora::core::ann_index::{
    ANNIndex,
    SerializableIndex
};
use hora::core::metrics::Metric;

use rusqlite::{
    Connection
};

use zip::read::ZipArchive;

use std::ffi::{
    OsString,
    OsStr
};
use std::path::{
    PathBuf
};

use std::io::{
    Result,
    Read,
    Seek
};
use std::fs::{
    Metadata,
    read_dir,
    ReadDir, 
    DirEntry,
    read_to_string,
    File
};

use std::collections::VecDeque;
use std::sync::Arc;

use crate::model::encode_text;
use crate::vectordb::{
    load_sqlite_db, 
    create_index,
    load_index
};
pub enum DocType{
    Pdf,
    Docx,
    Txt,
    Unsupported
}

#[derive(Debug,Clone)]
pub struct Document{
    name:String,
    metadata:Metadata,
    path:PathBuf,
    data:Option<Vec<String>>,
    encodings:Option<Vec<Vec<f32>>>
}

impl Document{
    pub fn is_supported(&self)->bool{
        let splits: (&str, &str) = self.name.rsplit_once('.').unwrap();
        let supported_types:[&str;3] = ["pdf","docx","txt"];
        supported_types.contains(&splits.1)
    }

    pub fn get_extension(&self)->Result<String>{
        Ok(self.name.rsplit_once(".").unwrap().1.to_string())
    }

    pub fn get_doc_type(&self,)->DocType{
        let extension: (&str, &str) = self.name.rsplit_once(".").unwrap();
        match extension.1{
            "pdf" => DocType::Pdf,
            "docx" => DocType::Docx,
            "txt" => DocType::Txt,
            _ => DocType::Unsupported
        }
    }

    pub fn get_document_path_as_string(&self)->Result<String>{
        Ok(self.path.clone().to_string_lossy().to_string())
    }

    pub fn get_document_name_as_string(&self)->Result<String>{
        Ok(self.name.clone())
    }

    pub fn set_document_data(&mut self, data:Vec<String>){
        self.data = Some(data);
    }

    pub fn encode_text_via_model(&self)->Option<Vec<Vec<f32>>>{
        let mut encodings:Vec<Vec<f32>> = vec![];
        let sentences:&Vec<String>;
        match &self.data{
            Some(vec_string)=>sentences=vec_string,
            None=>return None
        }

        match encode_text(sentences){
            Some(embedded_sentences)=>encodings=embedded_sentences,
            None=>{
                println!("Unable to generate Embeddings for document:{:?}",self.name);
                return None
            }
        }
        Some(encodings)
    }

}

pub trait document_uploads{
    fn encode_and_upload_documents(doc_list:&mut Vec<Document>, path:PathBuf);
}

impl document_uploads for Document{
    /*
Description: This function encodes the text of documents in a doc_list using a Sentence Embedding Model. Then returns those embeddings
encased in a vector. Make sure that the text in a document is split into chunks that the model can take as an input. 
    */
fn encode_and_upload_documents(doc_list:&mut Vec<Document>, path:PathBuf){
    
    let mut project_path = path.clone();
    project_path.push("cephalon.index");
    println!("{:?}",project_path);
    let mut index = create_index(project_path.clone(),384);

    let mut id:usize = 0;
    println!("doc_list_len: {:?}",doc_list.len());
    for doc in doc_list.iter(){
        println!("{:?} encoding and uploading to index",doc.get_document_name_as_string());
        let encodings:Vec<Vec<f32>>;


        match doc.encode_text_via_model(){
            Some(embeddings) => encodings = embeddings,
            None=>{
                continue
            }
        }
        

        let conn:Connection;
        match load_sqlite_db(&path){
            Some(db_conn)=>{
                conn = db_conn;
            },
            None=>{
                panic!("Unable to create a connection to sqlitedb");
            }
        }
        
        
        let doc_name:String;
        match doc.get_document_name_as_string(){
            Ok(doc_name_as_string)=> doc_name=doc_name_as_string,
            Err(err)=>panic!("Error getting document name {:}",err)
        }
        let len_sentence_encodings:usize = encodings.len();

        for i in 0..len_sentence_encodings{
            let encoding = &encodings[i];
            let sentence = &doc.data.as_ref().unwrap()[i];
            id+=1;
            match index.add(encoding,id){
                Ok(_msg)=>{
                    let params = (&doc_name, sentence, id.to_string());
                    match conn.execute("
                    INSERT INTO Vectors (DocumentName,Line,Label) VALUES (?1,?2,?3)
                    ", params.clone()){
                        Ok(_msg)=>{
                        },
                        Err(err)=>{
                            println!("Error Inserting data into sqlite: {:?}",err);
                        }
                    }
                },
                Err(err)=>{
                    panic!("Error inserting to index: {:?}",err)
                }
            }
        }
    }
    match index.build(Metric::Euclidean){
        Ok(_msg)=>{
            match index.dump(&project_path.to_str().unwrap()){
                Ok(_dump_msg)=>{},
                Err(dump_err)=>{
                    println!("{}",dump_err);
                }
            }
        },
        Err(err)=>{
            println!("{}",err);
        }
    }
}
}


/*
This function return True if the file format is supported by this program. It takes in OsStr and converts that
into a String type via lossy conversion. 
 */
fn is_supported(file_name:&OsStr)->bool{
    let split_str:String = file_name.to_string_lossy().to_string();
    let splits = split_str.rsplit_once('.').unwrap();
    let supported_types:[&str;3] = ["pdf","docx","txt"];
    supported_types.contains(&splits.1)
}

/*
Description: get_file_list() will get all the files in the current directory, create a Document object, and store it in a vector. 
This function will then return that vector.
 */
pub fn get_file_list(path:&PathBuf) ->Result<Vec<Document>> {
    let path_objects:ReadDir = read_dir(path).unwrap();
    let mut file_list:Vec<Document> = vec![];
    for path_object in path_objects{

        let object:DirEntry = path_object.unwrap();
        let file_metadata:Metadata = object.metadata().unwrap();
        let file_name:OsString = object.file_name();
        let file_path:PathBuf = object.path();

        if file_path.is_file() && is_supported(&file_name){
            match file_name.into_string(){
                Ok(fname)=>file_list.push(
                    Document { 
                        name:fname,
                        metadata:file_metadata,
                        path:file_path,
                        data:None,
                        encodings:None
                    }
                ),
                Err(os_str)=>file_list.push(
                    Document { 
                        name:os_str.to_string_lossy().to_string(),
                        metadata:file_metadata,
                        path:file_path,
                        data:None,
                        encodings:None
                    }
                ),
            }
        }
    }
    
    Ok(file_list)
    
}



/*
Given a Document, this function will determine if the Document is supported or not via the is_supported function from
the document, and if it is supported then call the appropriate text extraction function to extract text and return it as
an option. 
 */
pub fn get_file_text( doc:&Document)->Option<Vec<String>>{
    let file_path:String; 
    match doc.get_document_path_as_string(){
        Ok(doc_path)=> file_path=doc_path,
        Err(_err)=>return None
    }
    let file_text_option:Option<Vec<String>>;
    match doc.get_doc_type(){
        DocType::Pdf => file_text_option=get_text_from_pdf(file_path),
        DocType::Docx => file_text_option=get_text_from_docx(file_path),
        DocType::Txt => file_text_option=get_text_from_txt(file_path),
        DocType::Unsupported => return None
    }

    match file_text_option{
        Some(file_text)=>Some(file_text),
        None=>return None
    }
}


pub fn split_text_into_chunks(text:String, chunk_size:usize)->Result<Vec<String>>{
    let text_vector: Vec<String> = text.as_bytes().chunks(chunk_size).map(|chunk| String::from_utf8_lossy(chunk).to_string()).collect::<Vec<_>>();
    Ok(text_vector)
}

/*
Description: This function reads text form a .txt file and returns it as an Option<String>. 
Use Case: This primarily for internal/rust_api because it can be used with rayon since this is limited by the GIL. 
 */
pub fn get_text_from_txt(file_path:String)->Option<Vec<String>>{
    let text_result = read_to_string(file_path);
    let text_string:String;
    match text_result{
        Ok(text)=>text_string=text,
        Err(_err)=>return None
    }
    let text_vec_result = split_text_into_chunks(text_string, 256);
    match text_vec_result{
        Ok(text_vec)=>Some(text_vec),
        Err(_err)=> return None
    }
}


/*
This function aims to get data from a word file purely in rust. The file_path is passed in as string, 
and then from there using zip's ZipArchive the files within docx file are read. We get the document.xml file
and then using minidom we extract the text data from the file using breadth first traversal, and return it as string.
Use Case: This primarily for internal/rust_api because it can be used with rayon since this is limited by the GIL. 
TODO: Split the function between the zip file read and parsing xml to get text. 
 */

pub fn get_text_from_docx(file_path:String)->Option<Vec<String>>{
    let mut result: String = String::new();
    let mut xml_string:String = String::new();

    let file: File;
    match File::open(file_path){
        Ok(f)=>file=f,
        Err(e)=> return None
    }

    let mut zip_reader: ZipArchive<_>;
    match ZipArchive::new(file){
        Ok(zp)=> zip_reader = zp,
        Err(_err)=>return None
    }
    let mut document_xml_file: zip::read::ZipFile<'_>;
    match zip_reader.by_name("word/document.xml"){
        Ok(zpf)=> document_xml_file=zpf,
        Err(_err)=> return None
    }

    let outcome: std::result::Result<usize, std::io::Error> = document_xml_file.read_to_string(&mut xml_string);
    let element:Element = xml_string.parse().unwrap();
    let mut node_que:VecDeque<&Element> = VecDeque::new();
    let mut text_string:String = String::new();
    node_que.push_back(&element);

    while let Some(node) = node_que.pop_front(){//Breadth First Traversal of XML Tree
        if node.name() == "t"{
            result.push_str(&node.text());
            result.push_str("\n");
        }
        for child in node.children(){
            node_que.push_back(child);
        }
    }
    if result.len() == 0{//In case the string is empty
        result.push_str("   ");
    }
    let result_vec: Vec<String> = split_text_into_chunks(result, 256).unwrap();
    Some(result_vec)
}

/*
Description: This function extracts text from a pdf file file via the pdf_extract crate. It is written in rust, 
and there for can be used with Rayon for parallel processing. 
Use Case: This primarily for internal/rust_api because it can be used with rayon since this is limited by the GIL. 
 */
pub fn get_text_from_pdf(file_path:String)->Option<Vec<String>>{
    let result_string:String;
    let bytes: Vec<u8>;
    match std::fs::read(file_path){
        Ok(fs_bytes)=> bytes=fs_bytes,
        Err(_err)=>{
            println!("Error reading file: {:?}",_err);
            return None
        }
    }

    match pdf_extract::extract_text_from_mem(&bytes){
        Ok(pdf_text)=>result_string = pdf_text,
        Err(_err)=> return None
    }
    
    let result_string_vec = split_text_into_chunks(result_string, 256); 
    match result_string_vec{
        Ok(result_vec)=>Some(result_vec),
        Err(_err)=>None
    }
}