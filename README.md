# Cephalon Knowledge Base Assistant
Cephalon is a library to create NLP powered knowledge base assistant with privacy in mind.
Cephalon can provide:
* Single Source of all documentation. ✅
* Semantic Search ✅
* Multi-Modality [Schduled to start in late Fall 2023] ❔
    * Support for Images [Scheduled to start in late Fall 2023] ❔

------
# Creating a Knowledge Base Assistant 

You can create a knowledeg base with: 
```
cephalon init
```
or
```
cephalon create your-knowledge-base-assistant
```
After that move all the documentation that you might have into your knowledge base folder and run 
```
cephalon build
```

You can query the knowledge base by entering a query like this. 

```
cephalon answer 'your-query-or-text'
```
------
------
# Cephalon under the hood

Cephalon-rs is the base version of Cephalon purely written in Rust. It also uses other libraries such as serde, rayon, rust-bert, pdf-extract, minidom, and zip. It also uses clap to create the cli for Rust. For the index it uses the HNSW Index with default settings from hora-search.  

# Supported File Types

* PDF (.pdf) ✅
* Word Documents (.docx) ✅
* Text (.txt) ✅
* JSON [Scheduled for Summer 2023] ❔

