use std::path::PathBuf;

use serde::ser::{
    Serialize,
    Serializer
};


use hora::index::hnsw_idx::HNSWIndex;
use hora::index::{
    hnsw_params::HNSWParams
};
use hora::core::ann_index::ANNIndex;
use hora::core::metrics::Metric;
use hora::core::ann_index::SerializableIndex;



#[doc = "Create a HNSWIndex at location specified at path, with dimension dim"]
pub fn create_index(path:PathBuf, dim:usize)->HNSWIndex<f32,usize>{
    let mut index = HNSWIndex::<f32, usize>::new(
        dim,
        &HNSWParams::<f32>::default(),
    );

    index
}

/// Load a HNSWIndex from the location specified path.
pub fn load_index(path:PathBuf)->HNSWIndex<f32,usize>{
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


pub fn save_index(index:&mut HNSWIndex<f32,usize>,project_path:PathBuf){
    let mut path = project_path.clone();
    path.push("cephalon.index");

    match index.build(Metric::Euclidean){
        Ok(_msg)=>{
            match index.dump(&path.to_str().unwrap()){
                Ok(_dump_msg)=>{},
                Err(dump_err)=>{
                    println!("{}",dump_err);
                }
            }
        },
        Err(err)=>{
            println!("{}",err);
        }
    }
}


