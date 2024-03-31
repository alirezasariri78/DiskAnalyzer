mod dir;
mod node;
mod workers;

use crate::args::CommandArgs;
use dir::*;
use node::Node;
use std::path::PathBuf;
use std::sync::Arc;

use self::workers::ThreadPool;

pub fn get_tree(args: &CommandArgs) -> Arc<Node> {
    let thred_pool = Arc::new(ThreadPool::new(args.threads));

    let root = Arc::new(Node::new(PathBuf::new(), String::from("root"), 0));
    let device_names = 'A'..'Z';
    for d in device_names.filter(|d| drive_exists(*d)) {
        let mut path = d.to_owned().to_string();
        path.push_str(":\\");
        let dir_path = PathBuf::from(path);
        let node = Arc::new(Node::new(dir_path.clone(), d.to_string(), 0));
        root.add_child(&node);
        node.set_parent(&root);

        let cloned_args = args.clone();
        let inner_thread_pool = Arc::clone(&thred_pool);
        _ = &thred_pool.execute(move || {
            if let Err(e) = build_tree(dir_path, &node, cloned_args, &inner_thread_pool) {
                match e {
                    DirError::AccessDenied(path) => println!("Access To Path {path} Denied "),
                    _ => println!("Something Wen Wrong..."),
                }
            }
        });
    }
    root
}

fn build_tree(
    path: PathBuf,
    node: &Arc<Node>,
    args: CommandArgs,
    thred_pool: &ThreadPool,
) -> Result<(), DirError> {
    let dir_lis_result = path.read_dir();

    if let Err(e) = &dir_lis_result {
        return Err(DirError::AccessDenied(path.to_str().unwrap().to_string()));
    }

    for dir in dir_lis_result.unwrap() {
        let entry = dir.unwrap();
        let metadata = entry.metadata().unwrap();
        if metadata.is_dir() {
            let new_node = Arc::new(Node::new(
                entry.path().clone(),
                get_dir_lable(&entry.path()).to_string(),
                0,
            ));
            node.add_child(&new_node);
            new_node.set_parent(&node);
            let cloned_args = args.clone();
            match build_tree(entry.path(), &new_node, cloned_args, &thred_pool) {
                Ok(_) => (),
                Err(_) => (),
            }
        }
    }
    Ok(())
}
