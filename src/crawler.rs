mod dir;
mod node;

use crate::args::CommandArgs;
use dir::*;
pub use node::Node;
use std::path::PathBuf;
use std::sync::Arc;

pub fn get_tree(args: &CommandArgs) -> Arc<Node> {
    let root = Arc::new(Node::new(PathBuf::from("root"), String::from("root"), 0, 0));

    if args.drive.is_some() {
        build_drive_tree(args.drive.to_owned().unwrap(), &root);
    } else if args.path.is_some() {
        build_path_tree(args.path.to_owned().unwrap(), &root);
    } else {
        build_pc_tree(&root);
    }
    root
}

#[cfg(target_os = "windows")]
fn build_drive_tree(drives: Vec<String>, root: &Arc<Node>) {
    for d in drives {
        let mut path = d.to_owned().to_string();
        path.push_str(":\\");
        start_build(path, root);
    }
}

fn build_pc_tree(root: &Arc<Node>) {
    if cfg!(target_os = "windows") {
        let device_names = 'A'..'Z';
        build_drive_tree(
            device_names
                .filter(|d| drive_exists(*d))
                .map(|i| i.to_string())
                .collect(),
            root,
        );
    } else if cfg!(target_os = "linux") {
        build_path_tree("/".to_string(), root);
    }
}
fn build_path_tree(path: String, root: &Arc<Node>) {
    start_build(path, root);
}

fn start_build(path: String, root: &Arc<Node>) {
    let dir_path = PathBuf::from(path);
    let node = Arc::new(Node::new(
        dir_path.clone(),
        get_dir_lable(&dir_path).to_string(),
        0,
        Arc::clone(root).get_depth().get().to_owned() + 1,
    ));
    root.add_child(&node);
    node.set_parent(&root);
    let dir_size = get_dir_files_size(&dir_path);
    node.set_size(dir_size);
    if let Err(e) = build_tree(dir_path, &node) {
        match e {
            DirError::AccessDenied(path) => println!("Access To Path {path} Denied "),
            _ => println!("Something Wen Wrong..."),
        }
    }
}

fn build_tree(path: PathBuf, node: &Arc<Node>) -> Result<(), DirError> {
    let dir_lis_result = path.read_dir();
    if let Ok(dir_lis) = dir_lis_result {
        for dir in dir_lis {
            let entry = dir.unwrap();
            let metadata = entry.metadata().unwrap();
            if metadata.is_dir() {
                let new_node = Arc::new(Node::new(
                    entry.path().clone(),
                    get_dir_lable(&entry.path()).to_string(),
                    0,
                    Arc::clone(node).get_depth().get().to_owned() + 1,
                ));
                node.add_child(&new_node);
                new_node.set_parent(&node);
                let dir_size = get_dir_files_size(&entry.path());
                new_node.set_size(dir_size);
                match build_tree(entry.path(), &new_node) {
                    Ok(_) => (),
                    Err(_) => (),
                }
            }
        }
    } else if let Err(e) = dir_lis_result {
        println!("Exception in path :{:#?} :{:#?}", &path, e);
        return Err(DirError::AccessDenied(path.to_str().unwrap().to_string()));
    }

    Ok(())
}
