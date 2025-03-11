use itertools::Itertools;
use sha256::digest;

fn hasher<T: ToString>(data: T) -> String {
    let data_to_string = data.to_string();
    digest(data_to_string)
}

fn data_to_hashed_array<T: ToString + Clone>(data: &[T]) -> Vec<String> {
    data.iter().map(|d| hasher((*d).clone())).collect()
}

fn generate_merkle_tree<T: ToString + Clone>(data: &[T]) -> Vec<Vec<String>> {
    let mut hashed_data: Vec<String> = data_to_hashed_array(data);
    let mut merkle_tree: Vec<Vec<String>> = vec![];
    let mut data_length: usize = hashed_data.len();

    while data_length > 1 {
        if data_length % 2 == 1 {
            hashed_data.push(hashed_data.last().unwrap().to_string());
            merkle_tree.push(hashed_data.clone());
        }

        let mut next_layer: Vec<String> = vec![];

        for (current_hash, next_hash) in hashed_data.iter().tuple_windows().step_by(2) {
            let combined_hash = hasher(current_hash.to_owned() + next_hash);
            next_layer.push(combined_hash);
        }

        hashed_data = next_layer.clone();

        if next_layer.len() % 2 == 0 || next_layer.len() == 1 {
            merkle_tree.push(next_layer);
        }

        data_length = hashed_data.len();
    }

    merkle_tree

    // for (level, hashes) in merkle_tree.iter().enumerate() {
    //     println!("Level {}: {:?}", level, hashes);
    // }
}

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    // let res = data_to_hashed_array(&data);
    // println!("{:#?}", res);

    generate_merkle_tree(&data);
}
