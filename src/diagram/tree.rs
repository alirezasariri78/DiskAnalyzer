use crate::crawler::Node;
use std::{ops::Deref, sync::Arc};

const BRANCH_CHAR: &'static str = "├──";
const NODE_CHAR: &'static str = "└──";

pub fn create_tree_diagram(tree: &Arc<Node>) -> String {
    let mut result = String::new();
    crawl_tree(tree, &mut result);
    result
}

fn crawl_tree(tree: &Arc<Node>, result: &mut String) {
    let deref_tree = tree.deref();
    if deref_tree.get_name() != "root" {
        result.push_str(format!("{}{}", "\n", "\t").as_str());

        result.push_str(
            format!(
                "{}{}  ({})",
                NODE_CHAR,
                deref_tree.get_path().to_str().unwrap_or(""),
                deref_tree.get_size()
            )
            .as_str(),
        );
    } else {
        result.push_str("├──root (0)");
    }
    let childrens = tree.get_childes().lock().unwrap();
    let deref = childrens.deref();
    for (index, child) in deref.iter().enumerate() {
        let node = child.deref();
        result.push_str(
            format!(
                "{}{}{} ({})",
                '\n',
                BRANCH_CHAR,
                node.get_path().to_str().unwrap_or(""),
                node.get_size()
            )
            .as_str(),
        );
        crawl_tree(child, result);
    }
}
