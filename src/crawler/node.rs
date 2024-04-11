use std::path::PathBuf;
use std::sync::{Arc, Mutex, Weak};

#[derive(Debug)]
pub struct Node {
    name: String,
    size: Mutex<u64>,
    path: PathBuf,
    childrens: Mutex<Vec<Arc<Node>>>,
    parent: Mutex<Weak<Node>>,
}

impl Node {
    pub fn new(path: PathBuf, name: String, size: u64) -> Self {
        Self {
            size: Mutex::new(size),
            childrens: Mutex::new(Vec::new()),
            parent: Mutex::new(Weak::new()),
            name,
            path,
        }
    }

    pub fn get_size(&self) -> u64 {
        *self.size.lock().unwrap()
    }
    pub fn add_child(&self, node: &Arc<Node>) {
        _ = &self.childrens.lock().unwrap().push(Arc::clone(&node));
    }

    pub fn set_parent(&self, node: &Arc<Node>) {
        *self.parent.lock().unwrap() = Arc::downgrade(&node);
    }

    pub fn set_size(&self, size: u64) {
        let guard = &mut self.size.lock().unwrap();
        **guard += size;
        let parent = self.parent.lock().unwrap();
        if let Some(n) = parent.upgrade() {
            if n.name != "root" {
                n.set_size(size);
            }
        }
    }
}
