use rust_bert::pipelines::sentence_embeddings::{
    SentenceEmbeddingsBuilder,
    SentenceEmbeddingsModel, SentenceEmbeddingsModelType
};
/*
Description: Generating Vector encoding_model using sentence_transformers models using python. 
Use Case: As of now this function is used for internal, and python_api. 
TODO: Create a separate version for internal use cases. 
 */

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
        Ok(encodings)=>Some(encodings.to_owned()),
        Err(err)=>panic!("Error encoding text: {:?}",err)
    }
 }


/*
use std::fmt;
const CODE: &str = r#"
class ModelApi:
    def encode(self,model_name:str, text:str):
        try:
            from sentence_transformers import SentenceTransformer
            model = SentenceTransformer(model_name)
            encoding_models = model.encode(text)
            return encoding_models
        except Exception as e:
            print(e)
            return ""
model = ModelApi()
"#;


/*
Description: Generating Vector encoding_model using sentence_transformers models using python. 
Use Case: As of now this function is used for internal, and python_api. 
TODO: Create a separate version for internal use cases. 
 */

pub fn encode_text(model_name:&String, text:&String)->Option<Vec<f32>>{
    let mut encodings:Vec<f32> = vec![];
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let module:&PyModule;
        match PyModule::from_code(py,CODE,"",""){
            Ok(py_code)=>module=py_code,
            Err(err)=> panic!("Error importing python module: {:?}",err)
        }
        
        let model_api_instance:&PyAny;
        match module.getattr("model"){
            Ok(model_instance)=> model_api_instance=model_instance,
            Err(_err)=> panic!("Error loading model_api instance")
        }
    
        let args:(&String,&String) =(model_name, text,);
    
        let embedding:&PyAny;
        match model_api_instance.call_method1("encode",args){
            Ok(encoding) => embedding = encoding,
            Err(_err)=> panic!("Error generating encodings")
        }

        encodings = embedding.extract().unwrap();
    });
    println!("{:?}",encodings);
    Some(encodings)
}
*/

