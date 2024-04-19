use crate::crawler::Node;
use std::sync::Arc;
mod tree;

pub fn show_diagram(root: &Arc<Node>) {
    let str = tree::create_tree_diagram(root);
    println!("{str}");
}
