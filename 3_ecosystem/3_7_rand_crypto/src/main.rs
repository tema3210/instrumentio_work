use std::{collections::HashSet, io::BufRead, path::PathBuf};

use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use base64ct::Encoding;
use rand::rngs::OsRng;
use uuid::Uuid;

fn generate_password(alphabet: &[char],len: u8) -> String {
    let mut s = String::with_capacity(len as usize);
    for _ in 0..len {
        s.push(*select_rand_val(alphabet))
    };
    s.shrink_to_fit();
    s
}

fn select_rand_val<T>(slice: &[T]) -> &T {
    &slice[rand::random::<usize>() % slice.len()]
}

fn new_access_token() -> Uuid {
    Uuid::new_v4()
}

fn get_file_hash(path: PathBuf) -> String {
    use sha3::{Digest,Sha3_512};

    let mut hasher = Sha3_512::new();

    let file = std::fs::File::open(path).unwrap();

    const CAP: usize = 1024 * 128;
    let mut reader = std::io::BufReader::with_capacity(CAP, file);

    loop {
        let length = {
            let buffer = reader.fill_buf().unwrap();
            hasher.update(buffer);
            buffer.len()
        };
        if length == 0 {
            break;
        }
        reader.consume(length);
    }

    let hash = hasher.finalize();
    base64ct::Base64::encode_string(&hash) 


}


//use argon2
fn hash_password<S: AsRef<str>>(pass: S) -> Option<String> {

    let argon2 = Argon2::default();

    let salt = SaltString::generate(&mut OsRng);
    
    argon2.hash_password(pass.as_ref().as_bytes(), &salt)
        .ok().map(|h| h.to_string())
}




fn main() {
    println!("Implement me!");
}
