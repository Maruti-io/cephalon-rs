use std::path::PathBuf;
use rayon::{prelude::*, current_thread_index};

use serde::ser::{
    Serialize,
    Serializer
};


use hora::index::hnsw_idx::HNSWIndex;
use hora::index::{
    hnsw_params::HNSWParams
};
use hora::core::ann_index::SerializableIndex;

use rusqlite::{
    Connection, 
    Result
};

use crate::document::Document;




pub fn create_index(path:PathBuf, dimension:usize)->HNSWIndex<f32,usize>{
    let mut index = HNSWIndex::<f32, usize>::new(
        dimension,
        &HNSWParams::<f32>::default(),
    );

    match index.dump(path.to_str().unwrap().as_ref()){
        Ok(msg)=>{
            println!("Created Index file");
        },
        Err(err)=>{
            println!("Error saving index file to storage: {:?}",err);
        }
    }

    index
}

pub fn load_index(path:PathBuf,dimension:usize)->HNSWIndex<f32,usize>{
    let mut project_path:PathBuf = path.clone();
    project_path.push("cephalon.index");
    let mut index:HNSWIndex<f32,usize>;
    match HNSWIndex::<f32,usize>::load(project_path.to_str().unwrap()){
        Ok(hnsw)=>{
            index = hnsw;
        },
        Err(err)=>{
            panic!("Error loading Index: {:?}",err)
        }
    }
    
    index
}

/*
This function generates a new sqlite project at the specified project path.
 */
pub fn create_sqlite_db(path:PathBuf)->Connection{
    let mut project_path = path.clone();
    project_path.push("cephalon.db3");
    let conn:Connection;
    match Connection::open(project_path){
        Ok(db_conn)=>{
            conn = db_conn;
        },
        Err(err)=>{
            panic!("Error creating sqlitedb: {:?}",err)
        }
    }

    match conn.execute("
    CREATE TABLE IF NOT EXISTS Vectors(
        Id INTEGER PRIMARY KEY AUTOINCREMENT,
        DocumentName TEXT NOT NULL,
        Line TEXT,
        Label INTEGER NOT NULL
    )
    ", ()){
        Ok(result)=>{
            println!("Tables Created");
        },
        Err(err)=>{
            panic!("Error Generating Tables: {:?}",err);
        }
    }
    conn
}

/*
Description: This is function will create a connection to an existing sqlite database connection specified in the project path. 
 */
pub fn load_sqlite_db(path:&PathBuf)->Option<Connection>{
    let mut project_path = path.clone();
    project_path.push("cephalon.db3");
    match Connection::open(project_path){
        Ok(db_connection)=>Some(db_connection),
        Err(err)=>{
            panic!("Error loading connection from project: {:?}",err);
        }
    }
}