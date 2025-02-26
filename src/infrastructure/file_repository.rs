use crate::domain::models::FrenchAddress;
use serde_json;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::Read;

const FILE_PATH: &str = "addresses.json";

pub fn save_addresses(addresses: &Vec<FrenchAddress>) -> Result<(), Box<dyn Error>> {
    let file = File::create(FILE_PATH)?;
    serde_json::to_writer(file, &addresses)?;
    Ok(())
}

pub fn load_addresses() -> Vec<FrenchAddress> {
    let mut file = OpenOptions::new()
        .read(true)
        .open(FILE_PATH)
        .unwrap_or_else(|_| File::create(FILE_PATH).unwrap());

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    serde_json::from_str(&content).unwrap_or_else(|_| Vec::new())
}
