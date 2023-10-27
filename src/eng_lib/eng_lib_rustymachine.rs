
//external lib
use rusty_machine::linalg::{Matrix, Vector};
use rusty_machine::prelude::SupModel;
use rusty_machine::learning::lin_reg::LinRegressor;

use rusty_machine::learning::svm::SVM;
//use rusty_machine::learning::toolkit::kernel::HyperTan;

use rusty_machine::analysis::score::neg_mean_squared_error;


use crate::{EngineList, Dataset};

use std::error::Error;

pub struct EngLibRustyMachine {
    name: EngineList
}

impl EngLibRustyMachine {
    pub fn new(eng: EngineList) -> EngLibRustyMachine {

        EngLibRustyMachine {
            name: eng
        }
    } 

}

impl super::EngLibrary for EngLibRustyMachine {

    fn name(&self) -> &EngineList {&self.name}

    fn linear_regression(&mut self, dataset: Dataset) -> Result<(), Box<dyn Error>> {


        //println!("my data {}", &dataset.data);
        
        let loaded_data = &dataset.data.get_data(); 

        println!("getting the file name {}", &dataset.data.get_interface_file());

        let (train_size, test_size) = loaded_data.data_size;
        let (x_train, y_train)      = loaded_data.train_set.clone(); //todo: find a way to not need to clone here
        let (x_test, y_test)        = loaded_data.test_set.clone();

        
        let x_train = Matrix::new(train_size, 13, x_train);
        let y_train = Vector::new(y_train);
    
        let x_test = Matrix::new(test_size, 13, x_test);
        let y_test = Matrix::new(test_size, 1, y_test);
        // Create a linear regression model
        let mut lin_model = LinRegressor::default();
        //let ((x_train, y_train), (x_test, y_test)) = rusty_machine_pre_processing(&dataset);
        let _ = lin_model.train(&x_train, &y_train);
        let predictions = lin_model.predict(&x_test).unwrap();
        let predictions = Matrix::new(test_size, 1, predictions);
        let acc = neg_mean_squared_error(&predictions, &y_test);
        
        // show evaluation
        println!(">>>>error: {:?}", acc);
        println!(">>>>R2 score: {:?}\n\n", r_squared_score(&y_test.data(), &predictions.data()));
        
        Ok(())
    }

    
    fn svm(&mut self, dataset: Dataset) -> Result<(), Box<dyn Error>> {

        //println!("my data {}", &dataset.data);
        
        let loaded_data = &dataset.data.get_data(); 

        let (train_size, test_size) = &loaded_data.data_size;
        let (x_train, y_train)      = loaded_data.train_set.clone();
        let (x_test, y_test)        = loaded_data.test_set.clone();

        
        let x_train = Matrix::new(*train_size, 13, x_train);
        let y_train = Vector::new(y_train);
    
        let x_test = Matrix::new(*test_size, 13, x_test);
        let y_test = Matrix::new(*test_size, 1, y_test);

        // Create a svm model
        let mut svm_model = SVM::default();
        //let ((x_train, y_train), (x_test, y_test)) = rusty_machine_pre_processing(&dataset);

        let _ = svm_model.train(&x_train, &y_train);
        let predictions = svm_model.predict(&x_test).unwrap();
        let predictions = Matrix::new(*test_size, 1, predictions);
        let acc = neg_mean_squared_error(&predictions, &y_test);
        
        // show evaluation
        println!(">>>>error: {:?}", acc);
        println!(">>>>R2 score: {:?}\n\n", r_squared_score(&y_test.data(), &predictions.data()));
        
        Ok(())
    }


}



pub fn r_squared_score(y_test: &[f64], y_preds: &[f64]) -> f64 {
    let model_variance: f64 = y_test.iter().zip(y_preds.iter()).fold(
        0., |v, (y_i, y_i_hat)| {
            v + (y_i - y_i_hat).powi(2)
        }
    );

    // get the mean for the actual values to be used later
    let y_test_mean = y_test.iter().sum::<f64>() as f64
        / y_test.len() as f64;

    // finding the variance
    let variance =  y_test.iter().fold(
        0., |v, &x| {v + (x - y_test_mean).powi(2)}
    );
    let r2_calculated: f64 = 1.0 - (model_variance / variance);
    r2_calculated
}