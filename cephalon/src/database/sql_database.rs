use rusqlite::{
    Connection, 
};

use std::path::PathBuf;

use std::fmt;

type Result<T> = std::result::Result<T, SQLError>;

/// SQL Error
#[derive(Debug, Clone)]
pub struct SQLError;

// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
impl fmt::Display for SQLError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid sql transaction or connection")
    }
}

///This function generates a new sqlite project at the specified project path.
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


///Description: This is function will create a connection to an existing sqlite database connection specified in the project path. 
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

pub fn insert_data_into_sql_db(path:PathBuf,doc_name:&str,sentence:&str,id:usize)->Result<()>{
    let conn:Connection;
    match load_sqlite_db(&path){
        Some(sql_conn)=>{
            conn = sql_conn;
        },
        None=>{
            return Err(SQLError)
        }

    }
    let params = (doc_name, sentence, id.to_string());
                    match conn.execute("
                    INSERT INTO Vectors (DocumentName,Line,Label) VALUES (?1,?2,?3)
                    ", params.clone()){
                        Ok(_msg)=>{
                            Ok(())
                        },
                        Err(err)=>{
                            println!("Error Inserting data into sqlite: {:?}",err);
                            Err(SQLError)
                        }
                    }
}