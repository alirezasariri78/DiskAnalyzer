mod dir;
mod node;
mod workers;

use dir::*;
use node::Node;
use std::path::PathBuf;
use std::rc::Rc;

pub fn get_tree() -> Rc<Node> {
    let mut depth: u128 = 1;
    let root = Rc::new(Node::new(PathBuf::new(), String::from("root"), 0));
    let device_names = 'A'..'Z';
    for d in device_names {
        if drive_exists(d) {
            let mut path = d.to_owned().to_string();
            path.push_str(":\\");
            let dir_path = PathBuf::from(path);
            let node = Rc::new(Node::new(dir_path.clone(), d.to_string(), 0));
            root.add_child(&node);
            node.set_parent(&root);

            if let Err(e) = build_tree(dir_path, &node, &mut depth) {
                match e {
                    DirError::AccessDenied(path) => println!("Access To Path {path} Denied "),
                    _ => println!("Something Wen Wrong..."),
                }
            }
        }
    }
    root
}

fn build_tree(path: PathBuf, node: &Rc<Node>, depth: &mut u128) -> Result<(), DirError> {
    let dir_lis_result = path.read_dir();

    if let Err(e) = &dir_lis_result {
        return Err(DirError::AccessDenied(path.to_str().unwrap().to_string()));
    }
    if *depth == 0 {
        return Err(DirError::LastDepth);
    }
    *depth -= 1;

    for dir in dir_lis_result.unwrap() {
        let entry = dir.unwrap();
        let metadata = entry.metadata().unwrap();
        if metadata.is_dir() {
            let dir_path = entry.path();
            let dir_lable = dir_path.to_str().unwrap().split("\\").last().unwrap();

            let new_node = Rc::new(Node::new(dir_path.clone(), dir_lable.to_string(), 0));
            node.add_child(&new_node);
            new_node.set_parent(&node);

            match build_tree(dir_path, &new_node, depth) {
                Ok(_) => (),
                Err(_) => (),
            }
        }
    }
    Ok(())
}
