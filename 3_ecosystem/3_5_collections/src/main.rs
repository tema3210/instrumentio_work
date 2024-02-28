#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct User {
    id: String,
    name: String,
}

impl User {
    pub fn new<I: AsRef<str>, N: AsRef<str>>(id: I, name: N) -> Self {
        Self {
            id: id.as_ref().into(),
            name: name.as_ref().into(),
        }
    }
}

pub trait UserRepo {
    fn by_id<S: AsRef<str>>(&self, id: S) -> Option<User>;

    fn by_ids<'s, S: AsRef<str>>(
        &'s self,
        iter: impl Iterator<Item = S> + 's,
    ) -> impl Iterator<Item = User> + 's;

    fn search<'s, S: AsRef<str> + 's>(&'s self, alike: S) -> impl Iterator<Item = User> + 's;
}

pub struct ImmutCollection(im::HashMap<String, User>);

impl UserRepo for ImmutCollection {
    fn by_id<S: AsRef<str>>(&self, id: S) -> Option<User> {
        match self.0.iter().find(|(k, _)| *k == id.as_ref()) {
            Some((_, v)) => Some(v.clone()),
            None => None,
        }
    }

    fn by_ids<'s, S: AsRef<str>>(
        &'s self,
        iter: impl Iterator<Item = S> + 's,
    ) -> impl Iterator<Item = User> + 's {
        iter.filter_map(|s| self.by_id(s))
    }

    fn search<'s, S: AsRef<str> + 's>(&'s self, alike: S) -> impl Iterator<Item = User> + 's {
        self.0.iter().filter_map(move |(_, v)| {
            if v.name.contains(alike.as_ref()) {
                Some(v.clone())
            } else {
                None
            }
        })
    }
}

fn main() {
    println!("Implement me!");
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{ImmutCollection, User, UserRepo};

    fn mk_col() -> ImmutCollection {
        let col = ImmutCollection(im::hashmap! {
            "id1".into() => User::new("id1","name1"),
            "id2".into() => User::new("id2","name2"),
            "not_id".into() => User::new("not_id","you")
        });
        col
    }

    #[test]
    fn test_by_id() {
        let col = mk_col();

        let u = col.by_id("id1");

        assert_eq!(u, Some(User::new("id1", "name1")));
    }

    #[test]
    fn test_by_ids() {
        let col = mk_col();

        let us: Vec<_> = col.by_ids(["id1", "id2"].iter()).collect();

        assert_eq!(
            us,
            vec![User::new("id1", "name1"), User::new("id2", "name2")]
        );
    }

    #[test]
    fn test_search() {
        let col = mk_col();

        let found: HashSet<_> = col.search("name").collect();

        assert_eq!(
            found,
            HashSet::from([User::new("id1", "name1"), User::new("id2", "name2")])
        );
    }
}
