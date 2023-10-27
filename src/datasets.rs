use crate::data_lib::{DatasetLibrary, start_datalib};

use crate::DatasetList;



//#[derive(Clone)]
pub struct Dataset {
    pub       name: DatasetList,
    pub       data: Box<dyn DatasetLibrary>,

    
}

impl Dataset {      
    
    pub fn new(dataset : DatasetList) -> Dataset {
        Dataset {       name: dataset.clone(),
                        data: start_datalib(dataset), 
                }
    }

    pub fn load(&mut self) {
        
        let _ = self.data.load();

    }



}   
