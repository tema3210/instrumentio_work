use std::path::PathBuf;

fn select_rand_val<T>(slice: &[T]) -> &T {
    &slice[rand::random::<usize>() % slice.len()]
}

fn get_file_hash(path: PathBuf) -> String{
    unimplemented!();
}

fn hash_password<S: AsRef<str>>(pass: S) -> String {
    unimplemented!();
}

// newtype 'd be better
fn new_access_token() -> String {
    unimplemented!();
}

fn generate_password() -> String {
    unimplemented!()
}


fn main() {
    println!("Implement me!");
}
