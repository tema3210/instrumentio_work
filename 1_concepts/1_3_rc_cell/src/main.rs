use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

struct GlobalStack<T> {
    stack: Arc<Mutex<VecDeque<T>>>,
}

impl<T> Clone for GlobalStack<T> {
    fn clone(&self) -> Self {
        Self {
            stack: self.stack.clone(),
        }
    }
}

impl<T> GlobalStack<T> {
    pub fn push(&self, t: T) {
        self.stack
            .lock()
            .expect("failed to lock a mutex")
            .push_front(t)
    }

    pub fn try_push(&self, t: T) -> Result<(), &'static str> {
        self.stack
            .lock()
            .map_err(|_| "failed to lock a mutex")?
            .push_front(t);
        Ok(())
    }

    pub fn pop(&self) -> Option<T> {
        self.stack
            .lock()
            .map(|mut el| el.pop_front())
            .ok()
            .unwrap_or(None)
    }
}

fn main() {
    println!("Implement me!");
}
