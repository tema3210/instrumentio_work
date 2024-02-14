pub trait EventSourced<Ev: ?Sized> {
    fn apply(&mut self, event: &Ev);
}

/// inversion of EventSourced trait
/// Usage: whenever we get an Event that our `crate::user::User` can consume
/// we turn it into `Box<dyn ApplyEv<crate::user::User>>` and throw in `.apply(_)` call
/// Cost: allocation, but I suspect that in closed world assumption devirtualization can kick in
/// + arena allocators can handle if not
pub trait ApplyEv<T>
{
    fn apply(&self,to: &mut T);
}

/// the core impl
impl<T,Ev> ApplyEv<T> for Ev where T: EventSourced<Self> {
    fn apply(&self,to: &mut T) {
        EventSourced::apply(to, self)
    }
}

/// the dyn impl
impl<T> EventSourced<&dyn ApplyEv<T>> for T {
    fn apply(&mut self, event: &&dyn ApplyEv<T>) {
        event.apply(self)
    }
}

pub mod user {
    use std::time::SystemTime;

    use super::{event, EventSourced};


    /// this must have been a user repo
    #[derive(Debug)]
    pub struct User {
        pub id: Id,
        pub name: Option<Name>,
        pub online_since: Option<SystemTime>,
        pub created_at: CreationDateTime,
        pub last_activity_at: LastActivityDateTime,
        pub deleted_at: Option<DeletionDateTime>,
    }

    // there go per event impls

    impl EventSourced<event::UserCreated> for User {
        fn apply(&mut self, ev: &event::UserCreated) {
            let event::UserCreated { user_id, at } = ev;
            self.id = *user_id;
            self.created_at = *at;
            self.last_activity_at = LastActivityDateTime(at.0);
        }
    }

    impl EventSourced<event::UserNameUpdated> for User {
        fn apply(&mut self, ev: &event::UserNameUpdated) {
            let event::UserNameUpdated {
                name,
                user_id,
                at,
            } = ev;
            self.name = name.clone();
        }
    }

    impl EventSourced<event::UserBecameOnline> for User {
        fn apply(&mut self, ev: &event::UserBecameOnline) {
            let event::UserBecameOnline { user_id, at } = ev;
            self.online_since = Some(*at);
        }
    }

    impl EventSourced<event::UserBecameOffline> for User {
        fn apply(&mut self, ev: &event::UserBecameOffline) {
            let event::UserBecameOffline { user_id, at } = ev;
            self.online_since = None;
            self.last_activity_at = LastActivityDateTime(*at);
        }
    }

    impl EventSourced<event::UserDeleted> for User {
        fn apply(&mut self, ev: &event::UserDeleted) {
            let event::UserDeleted { user_id, at } = ev;
            self.deleted_at = Some(*at);
            self.last_activity_at = LastActivityDateTime(at.0);
        }
    }

    // fields    

    #[derive(Clone, Copy, Debug)]
    pub struct Id(pub u64);

    #[derive(Clone, Debug)]
    pub struct Name(pub Box<str>);

    #[derive(Clone, Copy, Debug)]
    pub struct CreationDateTime(pub SystemTime);

    #[derive(Clone, Copy, Debug)]
    pub struct LastActivityDateTime(pub SystemTime);

    #[derive(Clone, Copy, Debug)]
    pub struct DeletionDateTime(pub SystemTime);
}

pub mod event {

    //! So far, whenever we create a new Event, all the service subs who can consume it
    //! just implement the `EvenSourced` trait; then dispatch code in the 
    //! event consumption module gets comp time error that they cannot send the event to a service (during casting into a `Box<dyn ApplyEv<_>>`)
    //! 
    //! Or, we could slap `#[non_exhaustive]` on that event enum... =)

    use std::time::SystemTime;

    use super::user;

    #[derive(Debug)]
    pub struct UserCreated {
        pub user_id: user::Id,
        pub at: user::CreationDateTime,
    }

    #[derive(Debug)]
    pub struct UserNameUpdated {
        pub user_id: user::Id,
        pub name: Option<user::Name>,
        pub at: SystemTime,
    }

    #[derive(Debug)]
    pub struct UserBecameOnline {
        pub user_id: user::Id,
        pub at: SystemTime,
    }

    #[derive(Debug)]
    pub struct UserBecameOffline {
        pub user_id: user::Id,
        pub at: SystemTime,
    }

    #[derive(Debug)]
    pub struct UserDeleted {
        pub user_id: user::Id,
        pub at: user::DeletionDateTime,
    }



}
