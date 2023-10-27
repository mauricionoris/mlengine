use crate::{EngineList, Dataset, ModelList};

use std::error::Error;


pub struct EngLibScikitLearn {
    name: EngineList

}

impl EngLibScikitLearn {
    pub fn new(eng: EngineList) -> EngLibScikitLearn {

        EngLibScikitLearn {
            name: eng
        }
    } 

}


impl super::EngLibrary for EngLibScikitLearn {

    fn name(&self) ->&EngineList {&self.name}

    
    fn linear_regression(&mut self, dataset: Dataset) -> Result<(), Box<dyn Error>> {
        
        let interface_file = &dataset.data.get_interface_file();
        println!("getting the file name {}", interface_file);
        let _ = call_scikitlearn(interface_file, &ModelList::LinearRegression);

        println!("This are the metrics returned ");
        Ok(())
    }
    
    fn svm(&mut self, dataset: Dataset) -> Result<(), Box<dyn Error>> {
        
        let interface_file = &dataset.data.get_interface_file();
        println!("getting the file name {}", interface_file);
        let _ = call_scikitlearn(interface_file, &ModelList::SupportVectorMachine);

        println!("This are the metrics returned ");
        Ok(())
    }
    


}

use serde_json;
use std::process::Command;
use std::fs;

fn call_scikitlearn(interface_file: &String, model: &ModelList) {

    
    let mut binding = Command::new("/home/mnf/repo/rust/mlengine/python_handler.sh");
    let exec = binding.args([interface_file, &model.to_string(), "parameters"]);

    // Execute `ls` in the current directory of the program.
    exec.status().expect("process failed to execute");
    
    // --snip--
    println!("In file {}", interface_file);

    let data = fs::read_to_string(interface_file).expect("Unable to read file");
    let res: serde_json::Value = serde_json::from_str(&data).expect("Unable to parse");
    println!("{}", res)


}   