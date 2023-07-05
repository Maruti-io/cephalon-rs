
#[cfg(not(feature="no-ml"))]
type Result<T> = std::result::Result<T, SummaryModelError>;

/// SQL Error
#[cfg(not(feature="no-ml"))]
#[derive(Debug, Clone)]
pub struct SummaryModelError;

#[cfg(not(feature="no-ml"))]
pub fn generate_summary(input:String)->Result<Vec<String>>{
    use rust_bert::pipelines::common::{ModelResource, ModelType};
    use rust_bert::pipelines::summarization::{SummarizationConfig, SummarizationModel};
    use rust_bert::resources::RemoteResource;
    use rust_bert::t5::{T5ConfigResources, T5ModelResources, T5VocabResources};
    
    let config_resource = RemoteResource::from_pretrained(T5ConfigResources::T5_SMALL);
    let vocab_resource = RemoteResource::from_pretrained(T5VocabResources::T5_SMALL);
    let weights_resource = RemoteResource::from_pretrained(T5ModelResources::T5_SMALL);

    let summarization_config = SummarizationConfig::new(
        ModelType::T5,
        ModelResource::Torch(Box::new(weights_resource)),
        config_resource,
        vocab_resource,
        None,
    );
    let summarization_model: SummarizationModel;
    match SummarizationModel::new(summarization_config){
        Ok(summary_model)=>{
            summarization_model= summary_model;
        },
        Err(err)=>{
            return Err(SummaryModelError)
        }
    }
    let output = summarization_model.summarize(&[input]);
    Ok(output)
}