use rs_abbreviation_number::NumericAbbreviate;

use crate::args::CommandArgs;
use crate::crawler::Node;
use crate::util::*;
use std::{ops::Deref, sync::Arc};

const MIDDLE_CHAR: &'static str = "├──";
const END_CHAR: &'static str = "└──";

pub fn create_tree_diagram(tree: &Arc<Node>, args: &CommandArgs) -> String {
    let mut result = String::new();
    crawl_tree(tree, args, &mut result);
    result
}

fn crawl_tree(tree: &Arc<Node>, args: &CommandArgs, result: &mut String) {
    if tree.get_depth().get() == args.depth && args.depth != 0 {
        return;
    }
    if tree.get_depth().get() == 0 {
        result.push_str(add_branch(tree).as_str());
    }
    let childrens = tree.get_childes().borrow();
    let deref = childrens.deref();
    for child in deref {
        let node = child.deref();
        result.push_str(add_branch(node).as_str());
        crawl_tree(child, args, result);
    }
}

fn add_branch(node: &Node) -> String {
    let mut branch_char = MIDDLE_CHAR;
    if node.is_last_child() {
        branch_char = END_CHAR;
    }

    let mut size = node.get_size().get().abbreviate_number();
    size.push_str("iB");
    format!(
        "{}{}{}{} {}",
        '\n',
        "\t".repeat(node.get_depth().get()),
        branch_char,
        node.get_name(),
        format!(
            "{}({} bytes)",
            size,
            thousends_seperator(node.get_size().get()).as_str()
        )
    )
}
