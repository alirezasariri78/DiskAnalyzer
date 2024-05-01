mod table;
mod tree;
mod shared;


use cli_table::print_stdout;
use crate::crawler::Node;
use std::sync::Arc;


use crate::args::{CommandArgs, DiagramType};
pub fn show_diagram(root: &Arc<Node>, arguments: &CommandArgs) {
    match arguments.diagram {
        DiagramType::Tree => {
            let str = tree::create_tree_diagram(root, arguments);
            println!("{str}");
        }
        DiagramType::Table => {
            let table = table::create_table_diagram(root, arguments);
            print_stdout(table)
                .unwrap_or_else(|e| println!("Failed To Write Table:{}", e.to_string()));
        }
    };
}
