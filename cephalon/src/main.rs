

use cephalon::knowledge_base::{
    Cephalon,
    util,
};
use clap::{
    Args,
    Parser,
    Subcommand,
    Command
};


use std::path::PathBuf;

#[derive(Parser,Debug)]
#[clap(author,version, about)]
struct CommandArgs{
    #[clap(subcommand)]
    pub entity_type:EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType{
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
pub struct QueryKnowledgeBaseCommand{
    pub query:String,
}


fn main(){
    match CommandArgs::parse().entity_type{
        EntityType::create(project_name)=>{
            let current_dir_path:PathBuf = std::env::current_dir().unwrap();
            let cephalon_knowledge_base = Cephalon::new(current_dir_path);
        },
        EntityType::build(_)=>{
            let current_dir_path:PathBuf = std::env::current_dir().unwrap();
            let cephalon_knowledge_base = Cephalon::load(current_dir_path.clone());
            cephalon_knowledge_base.search_and_build_index(&current_dir_path);
        },
        EntityType::answer(query)=>{
            let current_dir_path:PathBuf = std::env::current_dir().unwrap();
            let cephalon_knowledge_base = Cephalon::load(current_dir_path.clone());
            cephalon_knowledge_base.search(current_dir_path, query.query,5);
        }
    }

}