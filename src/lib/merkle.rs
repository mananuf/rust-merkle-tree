use itertools::Itertools;
use sha256::digest;

/// todos:
/// 1 . handle errors
/// 2 . handle command using match expression
/// 3 . store merkle tree/root to file in pc
/// 3 . generate merkle proof logic and use merkle root data from file

fn hasher<T: ToString>(data: T) -> String {
    let data_to_string = data.to_string();
    digest(data_to_string)
}

fn data_to_hashed_array<T: ToString + Clone>(data: &[T]) -> Vec<String> {
    data.iter().map(|d| hasher((*d).clone())).collect()
}

pub fn generate_merkle_tree<T: ToString + Clone>(data: &[T]) -> Vec<Vec<String>> {
    let mut hashed_data: Vec<String> = data_to_hashed_array(data);
    let mut merkle_tree: Vec<Vec<String>> = vec![];
    let mut data_length: usize = hashed_data.len();

    while data_length > 1 {
        if data_length % 2 == 1 {
            hashed_data.push(hashed_data.last().unwrap().to_string());
        }

        let mut next_layer: Vec<String> = vec![];

        for (current_hash, next_hash) in hashed_data.iter().tuple_windows().step_by(2) {
            let combined_hash = hasher(current_hash.to_owned() + next_hash);
            next_layer.push(combined_hash);
        }

        merkle_tree.push(hashed_data.clone());
        hashed_data = next_layer.clone();

        if next_layer.len() == 1 {
            merkle_tree.push(next_layer);
        }

        data_length = hashed_data.len();
    }

    merkle_tree
}

pub fn get_merkle_root(tree: &Vec<Vec<String>>) -> String {
    tree.last().unwrap().last().unwrap().to_string()
}
