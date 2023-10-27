use std::{ fs::File, path::Path};
use rand::Rng;

use arrow2::{
    array::Array,
    chunk::Chunk,
    datatypes::{Schema}, 
    io::ipc::write,
};


use serde_arrow::arrow2::{serialize_into_arrays, serialize_into_fields};
use crate::data_lib::data_lib_bostonhousing::BostonHousing;

pub fn gen_arrow_file(content: &Vec<BostonHousing>)   -> Result<String, PanicOnError> {

    
    let fields = serialize_into_fields(&content, Default::default())?;
    let arrays = serialize_into_arrays(&fields, &content)?;

    let schema = Schema::from(fields);
    let chunk = Chunk::new(arrays);

    let p = "/tmp/".to_string();
   
    let filename = file_name_gen();
    
    let filepath = concat_string(p,filename);
   
    write_batches(&filepath, schema, &[chunk])?;

    Ok(filepath)
}

fn concat_string(a: String, b: String) -> String {
    a + &b
}

fn file_name_gen() -> String {
    
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";
    const FILENAME_LEN: usize = 40;
    let mut rng = rand::thread_rng();

    let filename: String = (0..FILENAME_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();


    println!("File generated {:?}", filename);
    filename


}

fn write_batches<P: AsRef<Path>>(
    path: P,
    schema: Schema,
    chunks: &[Chunk<Box<dyn Array>>],
) -> Result<(), PanicOnError> {
    let file = File::create(path)?;

    let options = write::WriteOptions { compression: None };
    let mut writer = write::FileWriter::new(file, schema, None, options);

    writer.start()?;
    for chunk in chunks {
        writer.write(chunk, None)?
    }
    writer.finish()?;
    Ok(())
}

#[derive(Debug)]
pub struct PanicOnError;

impl<E: std::fmt::Display> From<E> for PanicOnError {
    fn from(e: E) -> Self {
        panic!("Encountered error: {}", e);
    }
}


