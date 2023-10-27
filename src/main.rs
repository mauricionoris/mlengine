
use mlengine::enums::{ModelList, DatasetList, EngineList};
use mlengine::{Config};


fn main() {


    
    
    let cfg = Config {algorithm: ModelList::LinearRegression
                      , dataset: DatasetList::BostonHousing
                      , engine: EngineList::RustyMachine};
    let mle = cfg.build_engine();
    
    println!("{}",mle);
    mle.run();
    
    let cfg = Config {algorithm: ModelList::LinearRegression
                      , dataset: DatasetList::BostonHousing
                      , engine: EngineList::ScikitLearn};
    let mle = cfg.build_engine();

    println!("{}",mle);
    mle.run();

}
