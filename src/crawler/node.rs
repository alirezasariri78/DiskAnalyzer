use std::cell::{Cell, RefCell};
use std::path::PathBuf;
use std::rc::{Rc, Weak};

#[derive(Debug, Clone)]
pub struct Node {
    name: String,
    size: Cell<usize>,
    childrens: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
    path: PathBuf,
}

impl Node {
    pub fn new(path: PathBuf, name: String, size: usize) -> Self {
        Self {
            size: Cell::new(size),
            childrens: RefCell::new(Vec::new()),
            parent: RefCell::new(Weak::new()),
            name,
            path,
        }
    }

    pub fn add_child(&self, node: &Rc<Node>) {
        _ = &self.childrens.borrow_mut().push(Rc::clone(node));
    }

    pub fn set_parent(&self, node: &Rc<Node>) {
        *self.parent.borrow_mut() = Rc::downgrade(&node);
    }
}
