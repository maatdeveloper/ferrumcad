/* standard library */
use std::fs::File;
use std::io::{BufReader, BufWriter};

/* external crates */

/* ferrumcad crates */
use crate::cad::Model;

pub fn save_model(path: &str, model: &Model) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);

    serde_json::to_writer_pretty(writer, model)?;

    Ok(())
}

pub fn load_model(path: &str) -> Result<Model, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let model = serde_json::from_reader(reader)?;

    Ok(model)
}