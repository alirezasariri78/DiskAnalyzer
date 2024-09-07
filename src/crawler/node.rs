use std::cell::{Cell, RefCell};
use std::path::PathBuf;
use std::sync::{Arc, Weak};
use std::fmt;

#[derive(Debug)]
pub enum NodeType{
    Directory,
    Audio,
    Image,
    Video,
    File
}

#[derive(Debug)]
pub struct Node {
    name: String,
    size: Cell<u64>,
    depth: Cell<usize>,
    parent: RefCell<Weak<Node>>,
    path: PathBuf,
    childrens: RefCell<Vec<Arc<Node>>>,
    node_type:NodeType
}

impl Node {
    pub fn new(path: PathBuf, name: String, size: u64, depth: usize,node_type:NodeType) -> Self {
        Self {
            size: Cell::new(size),
            childrens: RefCell::new(Vec::new()),
            parent: RefCell::new(Weak::new()),
            depth: Cell::new(depth),
            name,
            path,
            node_type
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
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_depth(&self) -> &Cell<usize> {
        &self.depth
    }
    pub fn get_node_type(&self) -> &NodeType {
        &self.node_type
    }

    pub fn add_to_size(&self, size: u64) {
        let parent = &self.parent;
        let current_size = &self.get_size().get();
        _ = &self.get_size().set(current_size + size);
        if let Some(n) = parent.borrow().upgrade() {
            if n.name != "root" {
                n.add_to_size(size);
            }
        }
    }

    pub fn get_childes(&self) -> &RefCell<Vec<Arc<Node>>> {
        &self.childrens
    }
    pub fn is_last_child(&self) -> bool {
        if let Some(parent) = &self.parent.borrow().upgrade() {
            let childes = parent.childrens.borrow();
            return childes.iter().position(|c| **c == *self).unwrap_or(0) == childes.len() - 1;
        }
        false
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.path.to_str().unwrap_or("") == other.path.to_str().unwrap_or("")
    }
}

impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl  PartialEq for NodeType {
    fn eq(&self, other: &Self) -> bool {
        let first= &self.to_string();
        let second=&other.to_string();
        first==second
    }
}

mod tests {
    use std::{path::PathBuf, sync::Arc};

    use super::Node;

    #[allow(dead_code)]
    fn get_last_child() -> Arc<Node> {
        let root = Arc::new(Node::new(PathBuf::new(), "r".to_string(), 0, 0,super::NodeType::Directory));
        let last = Arc::new(Node::new(PathBuf::from("/c4"), "c4".to_string(), 4, 1,super::NodeType::Directory));
        let mut childes = vec![
            Arc::new(Node::new(PathBuf::from("/c1"), "c1".to_string(), 1, 1,super::NodeType::Directory)),
            Arc::new(Node::new(PathBuf::from("/c2"), "c2".to_string(), 2, 1,super::NodeType::Directory)),
            Arc::new(Node::new(PathBuf::from("/c3"), "c3".to_string(), 3, 1,super::NodeType::Directory)),
        ];
        childes.push(Arc::clone(&last));
        for c in &childes {
            c.set_parent(&root);
            root.add_child(c);
        }
        last
    }

    #[test]
    fn is_last_child_test() {
        let root = Arc::new(Node::new(PathBuf::new(), "r".to_string(), 0, 0,super::NodeType::Directory));
        let last = Arc::new(Node::new(PathBuf::from("/c4"), "c4".to_string(), 4, 1,super::NodeType::Directory));
        let mut childes = vec![
            Arc::new(Node::new(PathBuf::from("/c1"), "c1".to_string(), 1, 1,super::NodeType::Directory)),
            Arc::new(Node::new(PathBuf::from("/c2"), "c2".to_string(), 2, 1,super::NodeType::Directory)),
            Arc::new(Node::new(PathBuf::from("/c3"), "c3".to_string(), 3, 1,super::NodeType::Directory)),
        ];
        childes.push(Arc::clone(&last));
        for c in &childes {
            c.set_parent(&root);
            root.add_child(&c);
        }
        assert!(last.is_last_child());

        let new_last = Arc::new(Node::new(PathBuf::from("/c5"), "c5".to_string(), 5, 1,super::NodeType::Directory));
        new_last.set_parent(&root);
        root.add_child(&new_last);
        assert_eq!(false, last.is_last_child());
    }

    #[test]
    fn get_depth_test() {
        let last = get_last_child();
        assert_eq!(1, last.get_depth().get());
    }

    #[test]
    fn get_name_test() {
        let last = get_last_child();
        assert_eq!("c4", last.get_name());
    }

    #[test]
    fn get_size_test() {
        let last = get_last_child();
        assert_eq!(4, last.get_size().get());
    }

    #[test]
    fn add_to_size_test() {
        let last = get_last_child();
        assert_eq!(4, last.get_size().get());
        last.add_to_size(5);
        assert_eq!(9, last.get_size().get());
    }

    #[test]
    fn get_childes_test() {
        let root = Arc::new(Node::new(PathBuf::new(), "r".to_string(), 0, 0,super::NodeType::Directory));
        let childes = vec![
            Arc::new(Node::new(PathBuf::from("/c1"), "c1".to_string(), 1, 1,super::NodeType::Directory)),
            Arc::new(Node::new(PathBuf::from("/c2"), "c2".to_string(), 2, 1,super::NodeType::Directory)),
            Arc::new(Node::new(PathBuf::from("/c3"), "c3".to_string(), 3, 1,super::NodeType::Directory)),
        ];
        for c in &childes {
            c.set_parent(&root);
            root.add_child(c);
        }
        assert_eq!(childes, *root.get_childes().borrow());
    }
}
