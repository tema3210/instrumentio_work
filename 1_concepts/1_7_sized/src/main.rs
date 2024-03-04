use std::{borrow::Cow, collections::HashMap, sync::Mutex};

trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

#[derive(Clone)]
struct User {
    id: u64,
    email: Cow<'static, str>,
    activated: bool,
}

/// so far to use this over FFI we lack "only" a stable ABI
struct UserRepositoryDynamic {
    store: Mutex<Box<dyn Storage<u64, User>>>,
}

impl UserRepositoryDynamic {
    fn from_store(s: Box<dyn Storage<u64, User>>) -> Self {
        Self {
            store: Mutex::from(s),
        }
    }
}

trait UserRepository {
    fn set(&self, key: u64, val: User);

    fn get(&self, key: u64) -> Option<User>;

    fn remove(&self, key: u64) -> Option<User>;
}

impl UserRepository for UserRepositoryDynamic {
    fn set(&self, key: u64, val: User) {
        self.store.lock().map(|mut s| s.set(key, val));
    }

    fn get(&self, key: u64) -> Option<User> {
        self.store
            .lock()
            .map(|mut s| s.get(&key).cloned())
            .unwrap_or(None)
    }

    fn remove(&self, key: u64) -> Option<User> {
        self.store
            .lock()
            .map(|mut s| s.remove(&key))
            .unwrap_or(None)
    }
}

struct HashMapStorage {
    map: HashMap<u64, User>,
}

impl HashMapStorage {
    fn empty() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

impl Storage<u64, User> for HashMapStorage {
    fn set(&mut self, key: u64, val: User) {
        self.map.insert(key, val);
    }

    fn get(&self, key: &u64) -> Option<&User> {
        self.map.get(key)
    }

    fn remove(&mut self, key: &u64) -> Option<User> {
        self.map.remove(key)
    }
}

/// Marker trait for command, can be sealed
trait Command {}

struct CreateUser;

impl Command for CreateUser {}

/// the command handler API
trait CommandHandler<C: Command> {
    type Context: ?Sized;
    type Result;

    fn handle_command(&self, cmd: &C, ctx: &Self::Context) -> Self::Result;
}

impl CommandHandler<CreateUser> for User {
    type Context = dyn UserRepository;
    type Result = Result<(), &'static str>;

    fn handle_command(&self, _: &CreateUser, user_repo: &Self::Context) -> Self::Result {
        user_repo.set(self.id, self.clone());
        Ok(())
    }
}

fn main() {
    let store = HashMapStorage::empty();

    /// here dynamic cast happens
    let ctx = UserRepositoryDynamic::from_store(Box::new(store));

    let user = User {
        id: 5,
        email: "nooooo, we can fix this!".into(),
        activated: true, // yuumi eto chity (lol champ)
    };
    user.handle_command(&CreateUser, &ctx).expect("died");
}
