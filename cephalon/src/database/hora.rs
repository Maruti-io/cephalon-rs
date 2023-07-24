use serde::{
    Serialize,
    Serializer,
    Deserialize
};


use hora::index::hnsw_idx::HNSWIndex;
use hora::index::{
    hnsw_params::HNSWParams
};
use hora::core::ann_index::ANNIndex;
use hora::core::metrics::Metric;
use hora::core::ann_index::SerializableIndex;

use std::path::PathBuf;

use super::vectordb::utils;


#[derive(Debug,Serialize, Deserialize)]
pub struct HoraSearch{
    index_path:PathBuf,
    index_name:String,
    index_id:String,
}

impl HoraSearch{

    ///Returns the location where the HNSWIndex is stored
    pub fn get_index_path(self)->PathBuf{
        self.index_path
    }

    ///Returns the name of the HNSWIndex
    pub fn get_index_name(self)->String{
        self.index_name
    }

    ///Returns the ID of the HNSWIndex
    pub fn get_index_id(self)->String{
        self.index_name
    }

    /// Adds embeddings to the HNSWIndex
    pub fn add(){
    }

    ///Searches the HNSWIndex for items
    pub fn search(&self, embedding:&mut Vec<f32>, count:usize){

    }


}

impl utils for HoraSearch{
    fn create(path:PathBuf)->HoraSearch{

        let hora_index: HoraSearch = HoraSearch{
            index_path:path,
            index_name:"".to_string(),
            index_id:"".to_string(),
        };

        hora_index
    }

    fn load(path:PathBuf)->HoraSearch{
        let hora_index: HoraSearch = HoraSearch{
            index_path:path,
            index_name:"".to_string(),
            index_id:"".to_string(),
        };

        hora_index
    }

    fn save(s:Self){

    }
}