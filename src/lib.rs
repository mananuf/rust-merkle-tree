use std::{error::Error, process};
use std::path::Path;
use std::fs;

pub struct Config {
    pub command: String,
    pub file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Self {
        Self {
            command: args[1].clone(),
            file_path: args[2].clone()
        }
    }
}

pub fn parse_config(args: &[String]) -> Config {
    let config: Config = Config::new(&args);
    config
}

pub fn csv_reader<P: AsRef<Path>>(filename: P) -> Result<Vec<String>, Box<dyn Error>> {
    let file = fs::File::open(filename)?;
    let mut reader = csv::Reader::from_reader(file);
    let mut data: Vec<String> = vec![];

    for result in reader.records() {
        let record = result?;
        let record_to_string = record.iter().collect::<Vec<&str>>().join(",");
        data.push(record_to_string);
        
        // println!("{record_to_string}");
        // println!("{:?}", record);
    }

    Ok(data)
}