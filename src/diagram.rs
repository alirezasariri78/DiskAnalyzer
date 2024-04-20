use crate::crawler::Node;
use std::{env::args, sync::Arc};
mod tree;
use crate::args::{CommandArgs, DiagramType};
pub fn show_diagram(root: &Arc<Node>, arguments: &CommandArgs) {
    match arguments.diagram {
        DiagramType::tree => {
            let str = tree::create_tree_diagram(root,arguments);
            println!("{str}");
        }
        _ => (),
    };
}
