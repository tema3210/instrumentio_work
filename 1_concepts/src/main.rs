use std::{
    borrow::Borrow, fmt::{self, Display}, mem, ptr, sync::{Arc, Mutex}
};

/// Node represents a node in the doubly linked list
struct Node<T> {
    data: T,
    next: Option<Arc<Mutex<Node<T>>>>,
    prev: Option<Arc<Mutex<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Node {
            data,
            next: None,
            prev: None,
        }))
    }
}

/// DoublyLinkedList represents the thread-safe doubly linked list
pub struct DoublyLinkedList<T> {
    head: Option<Arc<Mutex<Node<T>>>>,
    tail: Option<Arc<Mutex<Node<T>>>>,
}

impl<T> DoublyLinkedList<T> {
    /// Creates an empty doubly linked list
    pub fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
        }
    }

    /// Adds a new element to the end of the list
    pub fn push_back(&mut self, data: T) {
        let new_node = Node::new(data);
        if let Some(ref mut tail) = self.tail {
            let tail_clone = tail.clone();
            let mut tail = tail.lock().unwrap();
            tail.next = Some(new_node.clone());
            new_node.lock().unwrap().prev = Some(tail_clone);
        } else {
            self.head = Some(new_node.clone());
        }
        self.tail = Some(new_node);
    }

    /// Removes and returns the element from the front of the list
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|head_arc| {
            let mut head = head_arc.lock().unwrap();
            if let Some(ref mut new_head) = head.next {
                new_head.lock().unwrap().prev = None;
                self.head = Some(new_head.clone());
            } else {
                self.tail = None;
            }
            drop(head);
            //we do this since we are the only owner left
            let inner = Arc::into_inner(head_arc).unwrap();

            let inner = inner.into_inner().unwrap();

            inner.data
        })
    }
}

impl<T: fmt::Display> fmt::Display for DoublyLinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DoublyLinkedList: [")?;

        let mut current = self.head.as_ref().map(|node| Arc::clone(node));

        while let Some(node) = current {
            let node = node.lock().unwrap();
            write!(f, "{}",node.data)?;

            current = node.borrow().next.as_ref().map(|next| Arc::clone(next));
            if current.is_some() {
                write!(f, " <-> ")?;
            }
        }

        write!(f, "]")
    }
}

fn main() {
    let mut list = DoublyLinkedList::new();

    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    let popped = list.pop_front();
    println!("Popped: {:?}", popped);

    println!("{}",&list);
}
