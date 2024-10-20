use crate::args::CommandArgs;
use crate::crawler::*;
use crate::diagram::shared::*;
use crate::util::*;
use colored::{Color, Colorize};
use rs_abbreviation_number::NumericAbbreviate;
use std::{ops::Deref, str::FromStr, sync::Arc};

pub const MIDDLE_CHAR: &'static str = "├──";
pub const END_CHAR: &'static str = "└──";

pub fn create_tree_diagram(tree: &Arc<Node>, args: &CommandArgs) -> String {
    let mut result = String::new();
    crawl_tree(tree, args, &mut result);
    result
}

fn crawl_tree(tree: &Arc<Node>, args: &CommandArgs, result: &mut String) {
    if *tree.get_node_type() != NodeType::Directory {
        return;
    }
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
        if *node.get_node_type() != NodeType::Directory {
            continue;
        }
        result.push_str(add_branch(node).as_str());
        crawl_tree(child, args, result);
    }
}

fn add_branch(node: &Node) -> String {
    let branch_char = get_branch_char(node);
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
            thousends_seperator(node.get_size().get()).as_str(),
        )
    )
}

fn get_branch_char(node: &Node) -> String {
    let mut branch_char = MIDDLE_CHAR;
    if node.is_last_child() {
        branch_char = END_CHAR;
    }
    let color_str = get_color_from_size(node.get_size().get());
    let colored = Color::from_str(color_str).unwrap_or(Color::White);
    branch_char.color(colored).to_string()
}

mod tests {

    use std::path::PathBuf;

    use super::*;
    #[test]
    fn get_branch_char_heavy_size_test() {
        let node = Node::new(
            PathBuf::from(""),
            "root".to_string(),
            1_000_000_000_000,
            0,
            NodeType::Directory,
        );
        assert_eq!("\u{1b}[31m├──\u{1b}[0m", get_branch_char(&node));
    }
    #[test]
    fn get_branch_char_medium_size_test() {
        let node = Node::new(
            PathBuf::from(""),
            "root".to_string(),
            8_024_000_000,
            0,
            NodeType::Directory,
        );
        assert_eq!("\u{1b}[33m├──\u{1b}[0m", get_branch_char(&node));
    }

    #[test]
    fn get_branch_char_small_size_test() {
        let node = Node::new(
            PathBuf::from(""),
            "root".to_string(),
            5,
            0,
            NodeType::Directory,
        );
        assert_eq!("\u{1b}[32m├──\u{1b}[0m", get_branch_char(&node));
    }

    #[test]
    fn add_branch_root_test() {
        let node = Node::new(
            PathBuf::from(""),
            "root".to_string(),
            5000,
            0,
            NodeType::Directory,
        );
        let input = add_branch(&node);
        let expect = "\n\u{1b}[32m├──\u{1b}[0mroot 5KiB(5,000 bytes)".to_string();
        assert_eq!(expect, input);
    }

    #[test]
    fn add_branch_child_test() {
        let child = Node::new(
            PathBuf::from(""),
            "child".to_string(),
            8_024_000_000,
            1,
            NodeType::Directory,
        );
        let input = add_branch(&child);
        let expect = "\n\t\u{1b}[33m├──\u{1b}[0mchild 8.02GiB(8,024,000,000 bytes)".to_string();
        assert_eq!(expect, input);
    }
}
