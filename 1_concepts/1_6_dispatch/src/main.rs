use std::borrow::Cow;

trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

struct User {
    id: u64,
    email: Cow<'static, str>,
    activated: bool,
}

/// this one can be used statically only, if rust team doesn't support Swift style generics (they expose generic API with sized generics at cost of implicit allocations)
struct UserRepositoryStatic<S: Storage<u64, User>> {
    store: S,
}

impl<S> From<S> for UserRepositoryStatic<S>
where
    S: Storage<u64, User>,
{
    fn from(value: S) -> Self {
        Self { store: value }
    }
}

impl<S> UserRepositoryStatic<S>
where
    S: Storage<u64, User>,
{
    fn set(&mut self, key: u64, val: User) {
        self.store.set(key, val)
    }
    fn get(&self, key: u64) -> Option<&User> {
        self.store.get(&key)
    }
    fn remove(&mut self, key: u64) -> Option<User> {
        self.store.remove(&key)
    }
}

/// so far to use this over FFI we lack "only" a stable ABI
struct UserRepositoryDynamic {
    store: Box<dyn Storage<u64, User>>,
}

impl UserRepositoryDynamic {
    fn from_boxed_store(b: Box<dyn Storage<u64, User>>) -> Self {
        Self { store: b }
    }

    fn set(&mut self, key: u64, val: User) {
        self.store.set(key, val)
    }
    fn get(&self, key: u64) -> Option<&User> {
        self.store.get(&key)
    }
    fn remove(&mut self, key: u64) -> Option<User> {
        self.store.remove(&key)
    }
}

fn main() {
    println!("Implement me!");
}
