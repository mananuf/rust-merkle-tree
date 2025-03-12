use std::env;
use std::fs;

pub mod merkle;
pub mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = lib::parse_config(&args);

    run(config);
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    let merkle_tree = merkle::generate_merkle_tree(&data);
    // println!("{:#?}", get_merkle_root(&merkle_tree));
    let res: String = merkle::get_merkle_root(&merkle_tree);
    println!("{res}");
}

fn run(config: lib::Config) {
    let content = fs::read_to_string(&config.file_path).expect("error reading file");
}
