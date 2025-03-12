use std::env;

pub mod merkle;
pub mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = lib::parse_config(&args);

    run(config);
}

fn run(config: lib::Config) {
    let data = lib::csv_reader(&config.file_path);
    let merkle_tree = merkle::generate_merkle_tree(&data.unwrap());
    println!("{:#?}", &merkle_tree);

    let merkle_root: String = merkle::get_merkle_root(&merkle_tree);
    println!("{merkle_root}");
}
