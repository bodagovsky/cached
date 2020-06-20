use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};
struct Node {
    val: i32,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Weak<RefCell<Node>>>,
}

impl Node {
    fn new(v: i32) -> Self {
        Node {
            val: v,
            next: None,
            prev: None
        }
    }
}
pub struct LRUCache {
    cap: usize,
    len: usize,
    key_holder: HashMap<i32, Rc<RefCell<Node>>>,
}

impl LRUCache {
    pub fn new(capacity: usize) -> Self {
        LRUCache {
            cap: capacity,
            len: 0,
            key_holder: HashMap::new(),
        }
    }

    pub fn length(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.cap
    }

    pub fn put(&mut self, key: i32, val:i32) {
        let n = Node::new(val);
        self.key_holder.insert(key, Rc::new(RefCell::new(n)));
    }

    pub fn get(&mut self, k:i32) -> i32{
        match self.key_holder.get(&k) {
            Some(v) => v.borrow().val,
            None => -1
        }
    }
}
