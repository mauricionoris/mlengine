pub mod enums;
mod algos;
mod datasets;
mod eng_lib;
mod data_lib;

use std::fmt;
use algos::Algorithm;
use datasets::Dataset;

use enums::{ModelList, DatasetList, EngineList};

pub struct Config {
    pub algorithm: ModelList,
    pub dataset: DatasetList,
    pub engine: EngineList
}

pub struct MlEngine  {
    pub algo: Algorithm,
    pub data: Dataset
    
}

impl Config {
    pub fn build_engine(self) -> MlEngine {
        MlEngine {
            algo : Algorithm::new(self.algorithm, self.engine),
            data : Dataset::new(self.dataset),
        }
    }
}

impl MlEngine {
    pub fn run(mut self) {
        
        self.data.load();
        self.algo.run(self.data);
    }
}

impl fmt::Display for MlEngine  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Selected Model: {}\nSelected Engine: {}\nSelected Dataset: {}", self.algo.model.to_string(),self.algo.engine.name().to_string(), self.data.name.to_string()) 
    }
}
