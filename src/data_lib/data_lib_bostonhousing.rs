use std::fs;
use std::fmt;
use std::error::Error;
use super::data_lib_common::Data;
use super::data_lib_arrow_writer;

use serde::Serialize;

#[derive(Debug)]
pub struct DataLibBostonHousing{
    pub data: Data,
    pub interface_file: String
}

impl DataLibBostonHousing {
    pub fn new() -> DataLibBostonHousing {
        DataLibBostonHousing {
           data: Data::new(),
           interface_file: String::from("")
        }
    }

}

impl Drop for DataLibBostonHousing {
    fn drop(&mut self) {

        let _ = fs::remove_file(&self.interface_file);
    }
}




impl fmt::Display for DataLibBostonHousing 
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        
    }
}


impl super::DatasetLibrary for DataLibBostonHousing {

    fn load(&mut self) -> Result<(), Box<dyn Error>> {

        let dataset_raw_content: Vec<BostonHousing> = load_bostonhousing();

        self.data = pre_process(&dataset_raw_content);

        //serialize
        self.interface_file = data_lib_arrow_writer::gen_arrow_file(&dataset_raw_content).unwrap();

        Ok(())





    }

    fn get_data(&self) -> &Data { &self.data }
    fn get_interface_file(&self) -> &String { &self.interface_file }


}






////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct BostonHousing {
    crim:     f64, // per capita crime rate by town
    zn:       f64, // proportion of residential land zoned for lots over 25,000 sq.ft.
    indus:    f64, // proportion of non-retail business acres per town.
    chas:     f64, // Charles River dummy variable (= 1 if tract bounds river; 0 otherwise).
    nox:      f64, // nitrogen oxides concentration (parts per 10 million).
    rm:       f64, // average number of rooms per dwelling.
    age:      f64, // proportion of owner-occupied units built prior to 1940.
    dis:      f64, // weighted mean of distances to five Boston employment centres.
    rad:      f64, // index of accessibility to radial highways.
    tax:      f64, // full-value property-tax rate per $10,000.
    ptratio:  f64, // pupil-teacher ratio by town.
    black:    f64, // 1000(Bk - 0.63)^2 where Bk is the proportion of blacks by town.
    lstat:    f64, // lower status of the population (percent).
    medv:     f64, // median value of owner-occupied homes in $1000s.
}


impl BostonHousing {
    fn new(v: Vec<&str>) -> BostonHousing {
        let f64_formatted: Vec<f64> = v.iter().map(|s| s.parse().unwrap()).collect();
        BostonHousing { crim: f64_formatted[0], 
                          zn: f64_formatted[1], 
                       indus: f64_formatted[2], 
                        chas: f64_formatted[3],
                         nox: f64_formatted[4], 
                          rm: f64_formatted[5], 
                         age: f64_formatted[6], 
                         dis: f64_formatted[7],
                         rad: f64_formatted[8], 
                         tax: f64_formatted[9], 
                     ptratio: f64_formatted[10], 
                       black: f64_formatted[11],
                       lstat: f64_formatted[12], 
                        medv: f64_formatted[13] }
    }

    pub fn into_feature_vector(&self) -> Vec<f64> {
        vec![self.crim, self.zn,       self.indus, self.chas, self.nox,
             self.rm,   self.age,      self.dis,   self.rad,
             self.tax,  self.ptratio,  self.black, self.lstat]
    }
    
    pub fn into_targets(&self) -> f64 {
        self.medv
    }
    





}


fn load_bostonhousing() -> Vec<BostonHousing> {
    println!("Loading BostonHousing");
    let file_path = "/home/mnf/repo/rust/mlengine/data/housing.csv";
    let data = get_boston_records_from_file(&file_path);
    println!("Load  BostonHousing Completed");

    data

}

fn get_boston_record(s: String) -> BostonHousing {
    let v: Vec<&str> = s.split_whitespace().collect();
    let b: BostonHousing = BostonHousing::new(v);
    b
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_boston_records_from_file(filename: impl AsRef<Path>) -> Vec<BostonHousing> {
    let mut v : Vec<BostonHousing> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            let line = line.unwrap(); //TODO: ERROR HANDLER
            let ds_item = get_boston_record(line);
            v.push(ds_item);
        }

    }
    v
}



fn pre_process(data: &Vec<BostonHousing>) -> Data {

    println!("Preprocessing BostonHousing");
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

    println!("Preprocessing BostonHousing Completed !");

    
    Data {
        train_set      : (boston_x_train, boston_y_train), 
        test_set       : (boston_x_test, boston_y_test),
        data_size      : (train_size, test_size)
    }
}
