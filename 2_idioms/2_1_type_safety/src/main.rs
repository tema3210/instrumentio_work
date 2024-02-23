use std::marker::PhantomData;

mod states {
    pub(crate) trait Sealed {}

    pub struct Unmoderated;
    impl Sealed for Unmoderated {}

    pub struct Published;
    impl Sealed for Published {}

    pub struct Deleted;
    impl Sealed for Deleted {}

    pub struct New;
    impl Sealed for New {}
}

mod post {
    #[derive(Clone, Debug, PartialEq)]
    pub struct Id(u64);

    #[derive(Clone, Debug, PartialEq)]
    pub struct Title(String);

    #[derive(Clone, Debug, PartialEq)]
    pub struct Body(String);
}

mod user {
    #[derive(Clone, Debug, PartialEq)]
    pub struct Id(u64);
}

#[derive(Clone)]
struct Post<State: states::Sealed> {
    id: post::Id,
    user_id: user::Id,
    title: post::Title,
    body: post::Body,
    _ph: PhantomData<State>,
}

impl Post<states::New> {
    fn publish(self) -> Post<states::Unmoderated> {
        Post {
            _ph: PhantomData,
            id: self.id,
            user_id: self.user_id,
            title: self.title,
            body: self.body,
        }
    }
}

impl Post<states::Unmoderated> {
    fn allow(self) -> Post<states::Published> {
        Post {
            _ph: PhantomData,
            id: self.id,
            user_id: self.user_id,
            title: self.title,
            body: self.body,
        }
    }
}

impl Post<states::Published> {
    fn delete(self) -> Post<states::Deleted> {
        Post {
            _ph: PhantomData,
            id: self.id,
            user_id: self.user_id,
            title: self.title,
            body: self.body,
        }
    }
}
impl Post<states::Deleted> {
    fn deny(self) -> Post<states::Unmoderated> {
        Post {
            _ph: PhantomData,
            id: self.id,
            user_id: self.user_id,
            title: self.title,
            body: self.body,
        }
    }
}

fn main() {
    println!("Implement me!");
}
