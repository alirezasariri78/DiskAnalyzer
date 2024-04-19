use crate::crawler::Node;
use std::sync::Arc;
mod tree;
use crate::args::{CommandArgs, DiagramType};
pub fn show_diagram(root: &Arc<Node>, arguments: &CommandArgs) {
    match arguments.diagram {
        DiagramType::tree => {
            let str = tree::create_tree_diagram(root);
            println!("{str}");
        }
        _ => (),
    };
}
