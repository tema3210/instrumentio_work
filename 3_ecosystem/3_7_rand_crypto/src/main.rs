use std::{io::BufRead, path::PathBuf};

use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use base64ct::Encoding;
use rand::{rngs::OsRng, seq::SliceRandom, Rng};

pub fn generate_password(alphabet: &[char], len: u8) -> String {

    struct Wrap<'s>(&'s [char]);

    impl<'s> rand::prelude::Distribution<char> for Wrap<'s> {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> char {
            let idx = rng.gen_range(0..self.0.len());
            self.0[idx]
        }
    }

    rand::thread_rng().sample_iter(Wrap(alphabet)).take(len.into()).collect()
}

pub fn select_rand_val<T>(slice: &[T]) -> &T {
    slice.choose(&mut rand::thread_rng()).expect("empty slice")
}

pub fn new_access_token() -> String {
    let mut rng = rand::thread_rng(); // it's also a CryptoRng
    let mut res = String::with_capacity(64);

    let ranges = ['0'..='9','A'..='Z','a'..='z'];

    for _ in 0..64 {
        let range = ranges.choose(&mut rng).unwrap().clone();
        let ch = rng.gen_range(range);
        res.push(ch);
    }

    res
}

pub fn get_file_hash(path: PathBuf) -> String {
    use sha3::{Digest, Sha3_512};

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
pub fn hash_password<S: AsRef<str>>(pass: S) -> Option<String> {
    let argon2 = Argon2::default();

    let salt = SaltString::generate(&mut OsRng);

    argon2
        .hash_password(pass.as_ref().as_bytes(), &salt)
        .ok()
        .map(|h| h.to_string())
}

fn main() {
    println!("Implement me!");
}
