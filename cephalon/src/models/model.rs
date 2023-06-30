///Description: Generating Vector encoding_model using sentence_transformers models using rust-bert. 
///Use Case: Use this to create embeddings for sentences.   
#[cfg(not(feature="no-ml"))]
pub fn encode_text(text:&Vec<String>)->Option<Vec<(String,Option<Vec<f32>>)>>{
    use rayon::prelude::*;
    use rust_bert::pipelines::sentence_embeddings::{
        SentenceEmbeddingsBuilder,
        SentenceEmbeddingsModel, SentenceEmbeddingsModelType
    };
    if text.len() == 0{
        println!("No text found");
        return None
    }

    let mut embedded_vector:Vec<Vec<(String,Option<Vec<f32>>)>> = vec![vec![],vec![],vec![],vec![]];
    
    //Using rayon parallel chunks split text into 4 chunks, and then get embeddings for each of the sentences in text. 
    text.par_chunks(4).zip(&mut embedded_vector).for_each(|(sentences, encoding_vec)| {
        let model:SentenceEmbeddingsModel;

        //Sentence Embedding Model
        match SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL6V2)
        .create_model(){
                Ok(sentence_embedding_model)=> model = sentence_embedding_model,
                Err(err)=>panic!("Error Generating Model: {:?}",err)
            }    
        
        //Iterate over each sentence in the chunk and create embeddings for it.
        for sentence in sentences{
            match model.encode(&[sentence]){
                Ok(encodings)=>{
                    //Create a tuple with original setting at 0, and embedding at 1, then push that tuple into encoding_vec
                    encoding_vec.push((sentence.to_owned(), Some(encodings[0].to_owned())));
                },
                Err(err)=>{
                    //In case of an error just push the original Sentence into a tuple, and then push None for embeddings
                    encoding_vec.push((sentence.to_owned(), None));
                    panic!("Error creating embeddings for text: {:?}",err)
                }
            }
        }  
    });

    let mut output_vec:Vec<(String,Option<Vec<f32>>)> = vec![];
    for mut slice in embedded_vector{
        output_vec.append(&mut slice);
    }

    Some(output_vec)
}