pub mod models;
pub mod documents;
pub mod knowledge_base;
pub mod database;

use documents::document::{
    split_text_into_chunks,
    get_text_from_pdf,
    get_text_from_docx,
    get_text_from_txt
};

use models::model::{
    encode_text
};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}


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
        let start_time = std::time::Instant::now();
        let _model_name = "all-MiniLM-L6-v2".to_string();
        let _result = encode_text(&vec![sentence]);
        println!("Time to generate embeddings: {:?}",start_time.elapsed());
    }

}
