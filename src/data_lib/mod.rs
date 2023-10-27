mod data_lib_arrow_writer;
mod data_lib_common;
mod data_lib_bostonhousing;
mod data_lib_mnist;

//common types
use self::data_lib_common::Data;

//specific datasets
use self::data_lib_bostonhousing::DataLibBostonHousing;
use self::data_lib_mnist::DataLibMNist;


use crate::{DatasetList};
use std::error::Error;

use std::fmt;

pub fn start_datalib(data: DatasetList) -> Box<dyn DatasetLibrary> {
    match data {
        DatasetList::BostonHousing => {
            Box::new(DataLibBostonHousing::new())
        },
        DatasetList::MNist => {
            Box::new(DataLibMNist::new())
        }
    }
}



pub trait DatasetLibrary: fmt::Display  {
    fn load(&mut self) -> Result<(), Box<dyn Error>>;
    
    fn get_data(&self) -> &Data;
    fn get_interface_file(&self) -> &String;
   


}





/*


trait Mytrait{

    //fn new(name: EngineList) -> Self;
}











*/