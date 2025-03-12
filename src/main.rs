use std::env;
use rust_merkle_tree::lib::merkle;
use rust_merkle_tree::lib::utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = utils::parse_config(&args);

    run(config);
}

fn run(config: utils::Config) {
    let data = utils::csv_reader(&config.file_path);
    let merkle_tree = merkle::generate_merkle_tree(&data.unwrap());
    println!("{:#?}", &merkle_tree);

    let merkle_root: String = merkle::get_merkle_root(&merkle_tree);
    println!("{merkle_root}");
}
