
use std::fmt;

#[derive(Debug)]
pub struct Data {
    pub train_set: (Vec<f64>,Vec<f64>),
    pub  test_set: (Vec<f64>, Vec<f64>),
    pub data_size: (usize, usize)
}


impl Data {
    pub fn new() -> Data {
        Data {
            train_set: (vec![0.64],vec![0.64]),
             test_set: (vec![0.64],vec![0.64]),
            data_size: (0,0)
        }
    }
}


impl fmt::Display for Data 
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (train_size, test_size) = &self.data_size;
        let (x_train, y_train)      = &self.train_set;
        let (x_test, y_test)        = &self.test_set;
        
        write!(f, "{:?} {:?} {:?} {:?} {:?} {:?}", train_size, test_size, x_train, y_train, x_test, y_test)
        //self.algo.algorithm.to_string(), self.data.name.to_string())
    }
}


//pub struct Meta {}