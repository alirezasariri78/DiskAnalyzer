use rs_abbreviation_number::NumericAbbreviate;

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
    if tree.get_name() == "root" {
        result.push_str(add_branch(tree).as_str());
    }
    let childrens = tree.get_childes().lock().unwrap();
    let deref = childrens.deref();
    for child in deref {
        let node = child.deref();
        result.push_str(add_branch(node).as_str());
        crawl_tree(child, result);
    }
}

fn add_branch(node: &Node) -> String {
    let mut size = node.get_size().abbreviate_number();
    size.push('B');
    format!(
        "{}{}{}{} ({})",
        '\n',
        "\t".repeat(node.get_depth()),
        BRANCH_CHAR,
        node.get_name(),
        size
    )
}
