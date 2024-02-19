use std::ops::Deref;

struct EmailString(String);

impl<'s> TryFrom<&'s str> for EmailString {
    type Error = &'static str; // or newtype for good tone

    fn try_from(value: &'s str) -> Result<Self, Self::Error> {
        // and here we'd like a lazy_static cell also
        let regex = regex::Regex::new(
            r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
        )
        .unwrap();
        if regex.is_match(value) {
            Ok(Self(String::from(value)))
        } else {
            Err("bad format")
        }
    }
}

impl Deref for EmailString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for EmailString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsRef<String> for EmailString {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

struct Random<T>([T; 3]);

impl<T> Deref for Random<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let idx = rand::random::<usize>() % 3;
        &self.0[idx]
    }
}

fn main() {
    println!("Implement me!");
}
