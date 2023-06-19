pub mod model;
pub mod document;
pub mod knowledge_base;
pub mod vectordb;

use document::{
    split_text_into_chunks,
    get_text_from_pdf,
    get_text_from_docx,
    get_text_from_txt
};

use model::{
    encode_text
};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

use std::time::Instant;

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
    fn encode_text_test(){
        let sentence = "This is a sentence that will be embedded!".to_string();
        let start_time = Instant::now();
        let model_name = "all-MiniLM-L6-v2".to_string();
        let result = encode_text(&model_name, &sentence);
        println!("Time to generate embeddings: {:?}",start_time.elapsed());
    }

    #[test]
    fn read_text_from_pdf_test(){

    }

}
