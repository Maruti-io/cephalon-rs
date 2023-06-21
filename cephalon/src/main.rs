

use cephalon::knowledge_base::{
    Cephalon,
    util, Matches,
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
    init(InitKnowledgeBaseCommand),
    create(CreateKnowledgeBaseCommand),
    build(BuildKnowledgeBaseCommand),
    answer(QueryKnowledgeBaseCommand),
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
    
}


#[derive(Debug,Args)]
pub struct QueryKnowledgeBaseCommand{
    pub query:String,
}


fn main(){
    match CommandArgs::parse().entity_type{
        EntityType::init(_)=>{
            let current_dir_path:PathBuf = std::env::current_dir().unwrap();
            let _cephalon_knowledge_base = Cephalon::new(current_dir_path);
        },
        EntityType::create(project_command)=>{
            let mut current_dir_path:PathBuf = std::env::current_dir().unwrap();
            current_dir_path.push(project_command.project_name);
            match std::fs::create_dir(&current_dir_path){
                Ok(_)=>{},
                Err(err)=>{
                    panic!("Error creating directory: {:?}",err)
                }
            }
            let _cephalon_knowledge_base = Cephalon::new(current_dir_path);
        },
        EntityType::build(_)=>{
            let current_dir_path:PathBuf = std::env::current_dir().unwrap();
            let cephalon_knowledge_base = Cephalon::load(current_dir_path.clone());
            cephalon_knowledge_base.search_and_build_index(&current_dir_path);
        },
        EntityType::answer(query)=>{
            let now = Instant::now();
            let current_dir_path:PathBuf = std::env::current_dir().unwrap();
            let cephalon_knowledge_base = Cephalon::load(current_dir_path.clone());
            let matches: Vec<Matches> = cephalon_knowledge_base.search(current_dir_path, query.query,5).unwrap();
            for search_result in matches{
                println!("{}, {:?}",search_result.document_name, search_result.line);
            }
            
            println!("Time to generate results: {:?}",now.elapsed().as_secs());
        }
    }

}