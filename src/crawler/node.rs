use std::cell::{Cell, RefCell};
use std::path::PathBuf;
use std::sync::{Arc, Weak};

#[derive(Debug)]
pub struct Node {
    name: String,
    size: Cell<u64>,
    depth: Cell<usize>,
    parent: RefCell<Weak<Node>>,
    path: PathBuf,
    childrens: RefCell<Vec<Arc<Node>>>,
}

impl Node {
    pub fn new(path: PathBuf, name: String, size: u64, depth: usize) -> Self {
        Self {
            size: Cell::new(size),
            childrens: RefCell::new(Vec::new()),
            parent: RefCell::new(Weak::new()),
            depth: Cell::new(depth),
            name,
            path,
        }
    }

    pub fn get_size(&self) -> &Cell<u64> {
        &self.size
    }
    pub fn add_child(&self, node: &Arc<Node>) {
        _ = &self.childrens.borrow_mut().push(Arc::clone(&node));
    }

    pub fn set_parent(&self, node: &Arc<Node>) {
        *self.parent.borrow_mut() = Arc::downgrade(node);
    }
    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_depth(&self) -> &Cell<usize> {
        &self.depth
    }

    pub fn set_size(&self, size: u64) {
        let parent = &self.parent;
        let current_size = &self.get_size().get();
        _ = &self.get_size().set(current_size + size);
        if let Some(n) = parent.borrow().upgrade() {
            if n.name != "root" {
                n.set_size(size);
            }
        }
    }

    pub fn get_childes(&self) -> &RefCell<Vec<Arc<Node>>> {
        &self.childrens
    }
}
