
use std::fmt;
use std::error::Error;
use super::data_lib_common::Data;


#[derive(Debug)]
pub struct DataLibMNist{
    pub data: Data,
    pub interface_file: String
}

impl DataLibMNist {
    pub fn new() -> DataLibMNist {
        DataLibMNist {
            data: Data::new(),
            interface_file: String::from("")
        }
    }
}

impl super::DatasetLibrary for DataLibMNist {
    fn load(&mut self) -> Result<(), Box<dyn Error>> {

        Ok(())
    }

    fn get_data(&self) -> &Data { &self.data }
    fn get_interface_file(&self) -> &String { &self.interface_file }

}





impl fmt::Display for DataLibMNist 
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        //self.algo.algorithm.to_string(), self.data.name.to_string())
    }
}


/*
fn load_mnist(&mut self)  {
    println!("Loadind MNIST");
    let file_path = "data/housing.csv";
    let data = get_boston_records_from_file(&file_path);

    let test_size: f64 = 0.2;
    let test_size: f64 = data.len() as f64 * test_size;
    let test_size = test_size.round() as usize;
    let (test_data, train_data) = data.split_at(test_size);
    let train_size = train_data.len();
    let test_size = test_data.len();

    let boston_x_train: Vec<f64> = train_data.iter().flat_map(|r| r.into_feature_vector()).collect();
    let boston_y_train: Vec<f64> = train_data.iter().map(|r| r.into_targets()).collect();

    let boston_x_test: Vec<f64> = test_data.iter().flat_map(|r| r.into_feature_vector()).collect();
    let boston_y_test: Vec<f64> = test_data.iter().map(|r| r.into_targets()).collect();
    
    self.train_set = (boston_x_train, boston_y_train); 
    self.test_set  = (boston_x_test, boston_y_test);

    self.data_size = (train_size, test_size);
    /*
    println!("Loaded all {:?} rows of the dataset", data.len());
    println!("An example of a row {:?}", data[0]);
    println!("feature vector {:?}", boston_x_train);
    println!("target vector {:?}", boston_y_train);
    */

    println!("Load MNist Completed");


}

*/