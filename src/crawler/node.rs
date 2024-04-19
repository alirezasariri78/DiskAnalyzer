use std::ops::Deref;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, Weak};

#[derive(Debug)]
pub struct Node {
    name: String,
    size: Mutex<u64>,
    depth: Mutex<usize>,
    parent: Mutex<Weak<Node>>,
    path: PathBuf,
    childrens: Mutex<Vec<Arc<Node>>>,
}

impl Node {
    pub fn new(path: PathBuf, name: String, size: u64, depth: usize) -> Self {
        Self {
            size: Mutex::new(size),
            childrens: Mutex::new(Vec::new()),
            parent: Mutex::new(Weak::new()),
            depth: Mutex::new(depth),
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
    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_depth(&self) -> usize {
        *self.depth.lock().unwrap()
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

    pub fn get_childes(&self) -> &Mutex<Vec<Arc<Node>>> {
        &self.childrens
    }
}
