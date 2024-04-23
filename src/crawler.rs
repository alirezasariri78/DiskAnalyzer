mod dir;
mod node;

use crate::args::CommandArgs;
use dir::*;
pub use node::Node;
use std::io::{self, ErrorKind};
use std::path::PathBuf;
use std::sync::Arc;

pub fn get_tree(args: &CommandArgs) -> Arc<Node> {
    let root = Arc::new(Node::new(PathBuf::from("root"), String::from("root"), 0, 0));

    if args.drive.is_some() {
        match args.drive.to_owned() {
            Some(drive) => {
                build_drive_tree(drive, &root);
                return root;
            }
            None => return root,
        }
    } else if args.path.is_some() {
        match args.path.to_owned() {
            Some(path) => {
                start_build(path, &root);
                return root;
            }
            None => return root,
        }
    } else {
        build_pc_tree(&root);
    }
    root
}

fn build_drive_tree(drives: Vec<String>, root: &Arc<Node>) {
    if cfg!(target_os = "windows") {
        for d in drives {
            let mut path = d.to_owned().to_string();
            path.push_str(":\\");
            start_build(path, root);
        }
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
        start_build("/".to_string(), root);
    }
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
            DirError::AccessDenied(path) => println!("Access To Path {} Denied.", path),
            _ => println!("UnhandledExeption"),
        }
    }
}

fn build_tree(path: PathBuf, node: &Arc<Node>) -> Result<(), DirError> {
    let dir_lis = path.read_dir()?;
    for dir in dir_lis {
        if let Ok(entry) = dir {
            if let Ok(metadata) = entry.metadata() {
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
                    if let Err(e) = build_tree(entry.path(), &new_node) {
                        println!("{:#?}", e)
                    }
                }
            }
        }
    }
    Ok(())
}

impl From<io::Error> for DirError {
    fn from(value: io::Error) -> Self {
        match value.kind() {
            ErrorKind::PermissionDenied => DirError::AccessDenied(value.to_string()),
            _ => DirError::UnhandledException,
        }
    }
}
