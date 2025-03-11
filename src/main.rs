use sha256::digest;

fn hasher<T: ToString>(data: T) -> String {
    let data_to_string = data.to_string();
    digest(data_to_string)
}

fn hash_data<T: ToString + Clone>(data: &[T]) -> Vec<String> {
    data.iter().map(|d| hasher((*d).clone())).collect()
}

fn main() {
    let data = vec![1, 2, 3, 4];
    let res = hash_data(&data);
    println!("{:#?}", res)
}
