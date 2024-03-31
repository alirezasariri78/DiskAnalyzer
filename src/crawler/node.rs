use std::borrow::BorrowMut;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, Weak};

#[derive(Debug)]
pub struct Node {
    name: String,
    size: Mutex<usize>,
    childrens: Mutex<Vec<Arc<Node>>>,
    parent: Mutex<Weak<Node>>,
    path: PathBuf,
}

impl Node {
    pub fn new(path: PathBuf, name: String, size: usize) -> Self {
        Self {
            size: Mutex::new(size),
            childrens: Mutex::new(Vec::new()),
            parent: Mutex::new(Weak::new()),
            name,
            path,
        }
    }

    pub fn add_child(&self, node: &Arc<Node>) {
        _ = &self.childrens.lock().unwrap().push(Arc::clone(&node));
    }

    pub fn set_parent(&self, node: &Arc<Node>) {
        *self.parent.lock().unwrap() = Arc::downgrade(&node);
    }
}
