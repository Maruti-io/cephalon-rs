
#[cfg(not(feature="no-ml"))]
use cephalon::knowledge_base::{
    Cephalon,
    Util, 
    Matches,
    DocumentEncoder
};
use clap::{
    Args,
    Parser,
    Subcommand,
    Command
};



use std::{path::PathBuf, time::Instant};

#[derive(Parser,Debug)]
#[clap(author,version, about)]
struct CommandArgs{
    #[clap(subcommand)]
    pub entity_type:EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType{
    Init(InitKnowledgeBaseCommand),
    Create(CreateKnowledgeBaseCommand),
    Build(BuildKnowledgeBaseCommand),
    Answer(QueryKnowledgeBaseCommand),
    Summarize(SummarizeDocumentCommand),
}

#[derive(Debug,Args)]
pub struct CreateKnowledgeBaseCommand{
    pub project_name:String,
}

#[derive(Debug,Args)]
pub struct BuildKnowledgeBaseCommand{
    
}

#[derive(Debug,Args)]
pub struct InitKnowledgeBaseCommand{
    #[arg(short='m',long="model")]
    model_path:Option<String>
}


#[derive(Debug,Args)]
pub struct QueryKnowledgeBaseCommand{
    pub query:String,
}


#[derive(Debug,Args)]
pub struct SummarizeDocumentCommand{
    pub file_path:String,
}

#[cfg(not(feature="no-ml"))]
fn main(){
    use cephalon::documents::document::Document;

    match CommandArgs::parse().entity_type{
        EntityType::Init(init_command)=>{
            match init_command.model_path{
                Some(model_path)=>{
                    let current_dir_path:PathBuf = std::env::current_dir().unwrap();
                    let _cephalon_knowledge_base: Cephalon = Cephalon::new(current_dir_path,true,model_path);
                },
                None=>{
                    let current_dir_path:PathBuf = std::env::current_dir().unwrap();
                    let _cephalon_knowledge_base: Cephalon = Cephalon::new(current_dir_path,false,"".to_string());
                }
            }
            
        },
        EntityType::Create(project_command)=>{
            let mut current_dir_path:PathBuf = std::env::current_dir().unwrap();
            current_dir_path.push(project_command.project_name);
            match std::fs::create_dir(&current_dir_path){
                Ok(_)=>{},
                Err(err)=>{
                    panic!("Error creating directory: {:?}",err)
                }
            }
            let _cephalon_knowledge_base = Cephalon::new(current_dir_path, false, "".to_string());
        },
        EntityType::Build(_)=>{
            let current_dir_path:PathBuf = std::env::current_dir().unwrap();
            let cephalon_knowledge_base = Cephalon::load(current_dir_path.clone());
            cephalon_knowledge_base.search_and_build_index(&current_dir_path);
        },
        EntityType::Answer(query)=>{
            let now = Instant::now();
            let current_dir_path:PathBuf = std::env::current_dir().unwrap();
            let cephalon_knowledge_base = Cephalon::load(current_dir_path.clone());
            let matches: Vec<(Matches)> = cephalon_knowledge_base.search(current_dir_path, query.query,5).unwrap();
            for search_result in matches{
                println!("{}, {:?}",search_result.document_name, search_result.line);
            }
            
            println!("Time to generate results: {:?}",now.elapsed().as_secs());
        }, 
        EntityType::Summarize(summarize_command)=>{
            let document:Document;
            match Document::load(summarize_command.file_path){
                Ok(doc)=>{
                    document=doc;
                },
                Err(err)=>{
                    panic!("Error loading document");
                }
            }
            match document.summarize(){
                Ok(summary)=>{
                    for sentence in summary.iter(){
                        println!("{}",sentence);
                    }
                    
                },
                Err(_err)=>{
                    println!("Error generating summary");
                }
            }
        }
    }

}

#[cfg(feature="no-ml")]
fn main(){

}