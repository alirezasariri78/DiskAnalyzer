use std::path::PathBuf;
use std::sync::{Arc, Mutex, Weak};

#[derive(Debug)]
pub struct Node {
    name: String,
    size: Mutex<u64>,
    childrens: Mutex<Vec<Arc<Node>>>,
    parent: Mutex<Weak<Node>>,
    path: PathBuf,
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

    pub fn add_child(&self, node: &Arc<Node>) {
        _ = &self.childrens.lock().unwrap().push(Arc::clone(&node));
    }

    pub fn set_parent(&self, node: &Arc<Node>) {
        *self.parent.lock().unwrap() = Arc::downgrade(&node);
    }

    pub fn set_size(&self, size: u64) {
        let guard = &mut self.size.lock().unwrap();
        **guard += size;
        if self.name != "root" {
            let parent = self.parent.lock().unwrap();
            if let Some(n) = parent.upgrade() {
                n.set_size(size);
            }
        }
    }
}
