# Cephalon- A Framework to build Machine Learning Applications
Cephalon is a framework to add machine learning capabilities such as semantic search systems, knowledge base assistants, and more. 
Cephalon can provide:
* Out of the box Semantic Search ✅
* Out of the box Knowledge Base Assistant ❔
* Multi-Modality [Schduled to start in late Fall 2023] ❔
    * Support for Images [Scheduled to start in late Fall 2023] ❔
* Support for masking private data ❔
* Single Source of all of your machine learning application needs at version 1.0. ❔

------
# Join us on our Adventure

Star us on [GitHub](https://github.com/Maruti-io/cephalon-rs)

Join us on [Discord](https://discord.gg/zYQdB3x9)

We would love to get some feedback from users on the project. We are working on developing a roadmap of the project as well. As such I would love to get some feed back from everyone, as to what features they would like to see in the project in the future. If there are features or issues you are facing please, let us know in the discord. We will do our best to respond to your questions as soon as possible! 

If you have some time please provide us with your feedback here: [Cephalon Roadmap Survey](https://forms.office.com/r/keAs3nK7kt)

------
# Installing Cephalon

Step 1: Install cephalon via ```cargo add cephalon```. 

Step 2: Install the libtorch library for enabling the use of pytorch models in rust. You can find the instructions to do so [here](https://github.com/LaurentMazare/tch-rs/blob/main/README.md)

# Installing Cephalon CLI

If you just want to play with the cli and test it without writing any code. You can install the CLI by

Step 1: Install the libtorch library for enabling the use of pytorch models in rust. You can find the instructions to do so [here](https://github.com/LaurentMazare/tch-rs/blob/main/README.md)

Step 2: Install the cephalon cli via: ```cargo install cephalon``` 

------
# Creating a Knowledge Base Assistant 

You can create a semantic search system with: 
```
cephalon init
```
or
```
cephalon create sample-sematic-search-app
```
After that move all the documentation that you might have into your project directory and run 
```
cephalon build
```

You can query the index by entering a query like this. 

```
cephalon answer 'your-query-or-text'
```

Create summaries of documents using the summarize command
```
cephalon summarize 'path\to\your\file'
```

------
# Using Cephalon in your code-base

## Creating a new cephalon project
```
use cephalon::knowledge_base::{
    Cephalon,
    util
};

fn main(){
    let current_dir_path:PathBuf = std::env::current_dir().unwrap();
    let cephalon = Cephalon::new(current_dir_path, false, "".to_string());
}
```
This will create  a .cephalon directory in the project directory. All, the data related to cephalon will be kept in there. 

## Scanning files and building Index and Database

```
use cephalon::knowledge_base::{
    Cephalon,
    util
};
fn main(){
    let current_dir_path:PathBuf = std::env::current_dir().unwrap();
    //Load and existing cephalon project
    let cephalon_semantic_search = Cephalon::load(current_dir_path.clone());
    //Point to the directory where the files are located. 
    cephalon_knowledge_base.search_and_build_index(&current_dir_path);
}
```
This will scan all the files in the given directory. Then if the file type is supported by the program, it will extract text from them, split it into chunks of 256 characters, and save it in the cephalon data base. It will also create embeddings for those files via a Sentence-Embedding model and then upload them to an index and save the index in .cephalon directory. At, the moment the files need to be in the same directory as .cephalon directory. However, in future it will allow you to index any file or directory from any path. 

## Searching for a specific text

```
use cephalon::knowledge_base::{
    Cephalon,
    util
};
fn main(){
    let current_dir_path:PathBuf = std::env::current_dir().unwrap();
    //Load a cephalon that is already built.
    let cephalon_semantic_search = Cephalon::load(current_dir_path.clone());

    //Search the Index and database for results
    let matches: Vec<Matches> = cephalon_semantic_search.search(current_dir_path, query.query,5).unwrap();

    //Iterate through matches and print them
    for search_result in matches{
        println!("{}, {:?}",search_result.document_name, search_result.line);
    }
}
```

------
# Cephalon under the hood

Cephalon-rs is the base version of Cephalon purely written in Rust. It also uses other libraries such as serde, rayon, rust-bert, pdf-extract, minidom, and zip. It also uses clap to create the cli for Rust. For the index it uses the HNSW Index with default settings from hora-search.  

# Supported File Types

* PDF (.pdf) ✅
* Word Documents (.docx) ✅
* Text (.txt) ✅
* JSON [Scheduled for late Fall 2023] ❔


