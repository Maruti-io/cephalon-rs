use cephalon;

use cephalon::document::{
    split_text_into_chunks,
    get_text_from_pdf,
    get_text_from_docx,
    get_text_from_txt,
    get_file_list,
};

use cephalon::model::{
    encode_text
};

use cephalon::vectordb::{
};

use cephalon::knowledge_base::{
    Cephalon,
    util
};


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

use std::time::Instant;
use std::path::PathBuf;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn split_text_into_chunks_test(){
        let result = split_text_into_chunks("Split this test please!!".to_string(), 5).unwrap();
        assert_eq!(result,vec!["Split"," this", " test"," plea","se!!"]);
    }   

    #[test]
    fn split_text_into_chunks_test_2(){
        let result = split_text_into_chunks("hello, world!".to_string(), 45).unwrap();
        assert_eq!(result,vec!["hello, world!"]);
    }    

    #[test]
    fn encode_text_test(){
        let sentence = vec!["Ok now I am writing a test sentence for my encoding procedure. This sentence will be encoded. I am also doing one more thing where I want to get up 256 characters because why not. Ok now I just need 100 more characters, how many of them did I get??🙂".to_string()];
        let start_time = Instant::now();
        let result = encode_text(&sentence);
        println!("Time to generate embeddings: {:?}",start_time.elapsed());
    }

    #[test]
    fn create_new_cephalon_test(){
        let mut project_path: PathBuf = std::env::current_dir().unwrap();
        project_path.push("tests");
        project_path.push("test_resources");
        println!("Path: {:?}",project_path.display());
        let cephalon = Cephalon::new(project_path);
    }

    #[test]
    fn get_file_list_test(){
        let mut project_path:PathBuf = std::env::current_dir().unwrap();
        project_path.push("tests");
        project_path.push("test_resources");
        let doc_list = get_file_list(&project_path).unwrap();
        assert_eq!(doc_list[0].get_document_name_as_string().unwrap(),"pdf-sample.pdf".to_string())
    }

    #[test]
    fn get_text_from_docx_test(){
        let mut project_path:PathBuf = std::env::current_dir().unwrap();
        project_path.push("tests");
        project_path.push("test_resources");
        project_path.push("word-docx-sample.docx");
        get_text_from_docx(project_path.to_string_lossy().to_string());
    }

}