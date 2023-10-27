
mod eng_lib_rustymachine;
mod eng_lib_scikitlearn;

//external engines 
use self::eng_lib_rustymachine::EngLibRustyMachine;
use self::eng_lib_scikitlearn::EngLibScikitLearn;


use crate::{EngineList, Dataset};
use std::error::Error;

pub fn start_englib(eng: EngineList) -> Box<dyn EngLibrary> {
    match eng {
        EngineList::RustyMachine => {
            Box::new(EngLibRustyMachine::new(eng))
        },
        EngineList::ScikitLearn => {
            Box::new(EngLibScikitLearn::new(eng))
        }
    }
}

pub trait EngLibrary {
    fn linear_regression(&mut self, dataset: Dataset) -> Result<(), Box<dyn Error>>;
    fn svm(&mut self, dataset: Dataset) -> Result<(), Box<dyn Error>>;

    fn name(&self) -> &EngineList;

}

/*
pub struct Parameters {
    test_size: f64,
    target   : String

}


*/






/*


trait Mytrait{

    //fn new(name: EngineList) -> Self;
}

struct Mystruct1{
    //pub name: EngineList
}

struct Mystruct2{}

impl Mytrait for Mystruct1 {
    
}

impl Mytrait for Mystruct2 {}









*/


//pub struct EngLibRustyMachine {}


//pub struct EngLibScikitLearn {}



/*

pub trait  Engine {

   // fn new( name: EngineList) -> Box<dyn Engine>;
    //fn name(&self) -> EngineList;
    fn regression(&mut self, dataset: Dataset) -> Result<(), Box<dyn Error>>;
}




impl dyn Engine {

    pub fn new(&self, name:EngineList)  -> Self {
        self.name = name;
    }

    pub fn regression(&mut self, dataset: Dataset) -> Result<(), Box<dyn Error>> {
        match self.name {
            EngineList::Rustymachine  => Rustymachine::regression(self, &dataset),
            EngineList::ScikitLearn => Self::start_scikit_learn(self),
        };

        Ok(())
    }




    pub fn random_forest(&mut self, _dataset: Dataset) -> Result<(), Box<dyn Error>> {

        Ok(())
    }




  //  fn start_rustymachine(&self) {}
  fn start_scikit_learn(&self) {}
    

    fn rusty_machine_pre_processing(&mut self, dataset: &Dataset) -> Box<((Matrix<T>,Vector<T>),(Matrix<T>,Matrix<T>))> {
        
        // pre processing the model with Rustymachine engine formats

       
        ((x_train, y_train), (x_test, y_test)) 

    }

    fn rusty_machine_pos_processing(&mut self) {


    }



    pub fn predict(&mut self) {

     // Create a linear regression model
     let mut lin_model = LinRegressor::default();
     // Now we will predict
     let predictions = lin_model.predict(&x_test).unwrap();

     predictions   
    

    }
    
        // move to other module
        // evaluate
    pub fn eval(&mut self) {

        let predictions = Matrix::new(*test_size, 1, predictions);
        let acc = neg_mean_squared_error(&predictions, &y_test);
        
        // show evaluation
        println!("linear regression error: {:?}", acc);
        println!("linear regression R2 score: {:?}", r_squared_score(&y_test.data(), &predictions.data()));

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
    


}

*/
