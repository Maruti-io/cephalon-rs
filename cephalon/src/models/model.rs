///Description: Generating Vector encoding_model using sentence_transformers models using rust-bert. 
///Use Case: Use this to create embeddings for sentences.   
#[cfg(not(feature="no-ml"))]
pub fn encode_text(text:&Vec<String>)->Option<Vec<Vec<f32>>>{
    use rayon::prelude::*;
    use rust_bert::pipelines::sentence_embeddings::{
        SentenceEmbeddingsBuilder,
        SentenceEmbeddingsModel, SentenceEmbeddingsModelType
    };
    if text.len() == 0{
        println!("No text found");
        return None
    }

    let mut embedded_vector:Vec<Vec<Vec<f32>>> = vec![vec![],vec![],vec![],vec![]];

    text.par_chunks(4).zip(&mut embedded_vector).for_each(|(sentences, encoding_vec)| {
        let model:SentenceEmbeddingsModel;

        match SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL6V2)
        .create_model(){
                Ok(sentence_embedding_model)=> model = sentence_embedding_model,
                Err(err)=>panic!("Error Generating Model: {:?}",err)
            }    
        
        for sentence in sentences{
            match model.encode(&[sentence]){
                Ok(mut encodings)=>{
                    encoding_vec.append(&mut encodings);
                },
                Err(err)=>{
                    panic!("Error encoding text: {:?}",err)
                }
            }
        }  
    });

    let mut output_vec:Vec<Vec<f32>> = vec![];
    for mut slice in embedded_vector{
        output_vec.append(&mut slice);
    }

    Some(output_vec)
}