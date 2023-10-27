
use crate::{ModelList,EngineList,Dataset};

use crate::eng_lib::{EngLibrary, start_englib};

pub struct Algorithm{
    pub model: ModelList, 
    pub engine: Box<dyn EngLibrary>
}

impl Algorithm {

    pub fn new( model: ModelList, engine_library: EngineList) -> Algorithm {

        Algorithm {
               model: model,
               engine: start_englib(engine_library)
    }
}
    
    pub fn run(&mut self, dataset: Dataset) {
        
        let _ = match self.model {
            ModelList::LinearRegression   => {
              let _ = self.engine.linear_regression(dataset);




            },
            
            
            ModelList::SupportVectorMachine => {
               let _ = self.engine.svm(dataset);

            }
            
            
        };
    }
 

}

