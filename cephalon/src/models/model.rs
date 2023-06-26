use rust_bert::pipelines::sentence_embeddings::{
    SentenceEmbeddingsBuilder,
    SentenceEmbeddingsModel, SentenceEmbeddingsModelType
};


///Description: Generating Vector encoding_model using sentence_transformers models using rust-bert. 
///Use Case: Use this to create embeddings for sentences.   
pub fn encode_text(text:&Vec<String>)->Option<Vec<Vec<f32>>>{
    if text.len() == 0{
        println!("No text found");
        return None
    }
    let model:SentenceEmbeddingsModel;
    match SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL6V2)
    .create_model(){
            Ok(sentence_embedding_model)=> model = sentence_embedding_model,
            Err(err)=>panic!("Error Generating Model: {:?}",err)
        }

    match model.encode(text){
        Ok(encodings)=>{
            Some(encodings)
        },
        Err(err)=>{
            panic!("Error encoding text: {:?}",err)
        }
    }  
}