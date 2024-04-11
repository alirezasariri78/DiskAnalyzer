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
    // for d in device_names.filter(|d| drive_exists(*d)) {
    //     let mut path = d.to_owned().to_string();
    //     path.push_str(":\\");
    //     let dir_path = PathBuf::from(path);
    //     let node = Arc::new(Node::new(dir_path.clone(), d.to_string(), 0));
    //     root.add_child(&node);
    //     node.set_parent(&root);

    //     let inner_thread_pool = Arc::clone(&thred_pool);
    //     _ = &thred_pool.execute(move || {
    //         if let Err(e) = build_tree(dir_path, &node, &inner_thread_pool, &mut 10) {
    //             match e {
    //                 DirError::AccessDenied(path) => println!("Access To Path {path} Denied "),
    //                 _ => println!("Something Wen Wrong..."),
    //             }
    //         }
    //     });
    // }

    for d in device_names.filter(|d| drive_exists(*d)) {
        let mut path = d.to_owned().to_string();
        path.push_str(":\\");
        let dir_path = PathBuf::from(path);
        println!("going pass dir_path: {:#?}", dir_path.clone());
        let node = Arc::new(Node::new(dir_path.clone(), d.to_string(), 0));
        root.add_child(&node);
        node.set_parent(&root);
        let dir_size = get_dir_files_size(&dir_path);
        node.set_size(dir_size);
        let inner_thread_pool = Arc::clone(&thred_pool);
        _ = &thred_pool.execute(move || {
            if let Err(e) = build_tree(dir_path, &node, &inner_thread_pool) {
                match e {
                    DirError::AccessDenied(path) => println!("Access To Path {path} Denied "),
                    _ => println!("Something Wen Wrong..."),
                }
            }
        });
    }

    root
}

fn build_tree(path: PathBuf, node: &Arc<Node>, thred_pool: &ThreadPool) -> Result<(), DirError> {
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
                ));
                node.add_child(&new_node);
                new_node.set_parent(&node);
                let dir_size = get_dir_files_size(&entry.path());
                new_node.set_size(dir_size);
                match build_tree(entry.path(), &new_node, &thred_pool) {
                    Ok(_) => (),
                    Err(_) => (),
                }
            }
        }
    } else {
        println!("Exceptio");
        return Err(DirError::AccessDenied(path.to_str().unwrap().to_string()));
    }

    Ok(())
}
