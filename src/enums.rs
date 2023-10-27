use std::fmt;


#[derive(Debug)]
pub enum ModelList {
    LinearRegression, SupportVectorMachine // Kmeans, Xgboost,
}

#[derive(Debug, Clone)]
pub enum DatasetList {
    BostonHousing,  MNist,
}

#[derive(Debug)]
pub enum EngineList {
    RustyMachine,  ScikitLearn,
}



impl fmt::Display for ModelList
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for DatasetList
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


impl fmt::Display for EngineList
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}