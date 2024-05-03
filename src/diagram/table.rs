use crate::crawler::Node;
use crate::{
    args::{CommandArgs, SortType},
    util::thousends_seperator,
};
use cli_table::Color;
use cli_table::{format::Justify, Cell, CellStruct, Style, Table, TableStruct};
use rs_abbreviation_number::NumericAbbreviate;
use std::str::FromStr;
use std::{ops::Deref, sync::Arc};

use super::shared::get_color_from_size;

pub fn create_table_diagram(tree: &Arc<Node>, args: &CommandArgs) -> TableStruct {
    let mut nodes: Vec<Arc<Node>> = Vec::new();
    crawl_tree(tree, args, &mut nodes);
    let data = export_to_table(nodes, args);
    data.table()
        .title(vec![
            "Name".cell().justify(Justify::Center).bold(true),
            "Size".cell().justify(Justify::Center).bold(true),
        ])
        .bold(true)
}

fn export_to_table(nodes: Vec<Arc<Node>>, args: &CommandArgs) -> Vec<Vec<CellStruct>> {
    let mut result: Vec<Vec<CellStruct>> = Vec::new();
    let sorted_data = sort_data(&nodes, args.sort.clone());
    for (name, size) in sorted_data {
        let formated_size = format!(
            "{}iB ({} bytes)",
            size.abbreviate_number(),
            thousends_seperator(size)
        );
        let color=Color::from_str(get_color_from_size(size)).unwrap_or(Color::White);
        result.push(vec![
            name.cell().foreground_color(Some(color)).justify(Justify::Center),
            formated_size.cell().foreground_color(Some(color)).justify(Justify::Center),
        ]);
    }
    result
}

fn crawl_tree(tree: &Arc<Node>, args: &CommandArgs, result: &mut Vec<Arc<Node>>) {
    if tree.get_depth().get() == args.depth && args.depth != 0 {
        return;
    }
    let childrens = tree.get_childes().borrow();
    let deref = childrens.deref();
    for child in deref {
        result.push(child.clone());
        crawl_tree(child, args, result);
    }
}

fn sort_data(input: &Vec<Arc<Node>>, sort_type: SortType) -> Vec<(&str, u64)> {
    let mut data: Vec<(&str, u64)> = input
        .iter()
        .map(|i| (i.get_name().as_str(), i.get_size().get()))
        .collect();

    match sort_type {
        SortType::Name(t) => {
            if !t {
                data.sort_by(|a, b| a.0.cmp(b.0));
            } else {
                data.sort_by(|a, b| b.0.cmp(a.0));
            }
            data
        }
        SortType::Size(t) => {
            if !t {
                data.sort_by(|a, b| a.1.cmp(&b.1));
            } else {
                data.sort_by(|a, b| b.1.cmp(&a.1));
            }
            data
        }
        _ => data,
    }
}

mod tests {

    use std::path::PathBuf;

    use super::*;

    #[allow(dead_code)]
    fn get_mock_data() -> Vec<Arc<Node>> {
        let mut mock = Vec::new();
        let names = ["a", "b", "c", " "];
        let sizes = [12, 1_000_000, 0, 12300];
        for (size, name) in names.iter().enumerate() {
            mock.push(Arc::new(Node::new(
                PathBuf::new(),
                name.to_string(),
                sizes[size],
                size,
            )));
        }
        mock
    }

    #[test]
    fn sort_by_name_desc_test() {
        let data = get_mock_data();
        let sorted_by_name = vec![("c", 0), ("b", 1_000_000), ("a", 12), (" ", 12300)];
        assert_eq!(sorted_by_name, sort_data(&data, SortType::Name(true)))
    }

    #[test]
    fn sort_by_name_asc_test() {
        let data = get_mock_data();
        let sorted_by_name = vec![(" ", 12300), ("a", 12), ("b", 1_000_000), ("c", 0)];
        assert_eq!(sorted_by_name, sort_data(&data, SortType::Name(false)))
    }

    #[test]
    fn sort_by_size_desc_test() {
        let data = get_mock_data();
        let sorted_by_size = vec![("b", 1_000_000), (" ", 12300), ("a", 12), ("c", 0)];

        assert_eq!(sorted_by_size, sort_data(&data, SortType::Size(true)))
    }

    #[test]
    fn sort_by_size_asc_test() {
        let data = get_mock_data();
        let sorted_by_size = vec![("c", 0), ("a", 12), (" ", 12300), ("b", 1_000_000)];
        assert_eq!(sorted_by_size, sort_data(&data, SortType::Size(false)))
    }

    #[test]
    fn crawl_tree_test() {
        let mut result: Vec<Arc<Node>> = Vec::new();
        let root = Arc::new(Node::new(PathBuf::new(), "r".to_string(), 0, 0));

        let mut childes = Vec::new();
        let childes_node = vec![
            Node::new(PathBuf::new(), "c1".to_string(), 1, 1),
            Node::new(PathBuf::new(), "c2".to_string(), 2, 1),
            Node::new(PathBuf::new(), "c3".to_string(), 3, 1),
            Node::new(PathBuf::new(), "c4".to_string(), 4, 1),
        ];
        for c in childes_node {
            childes.push(Arc::new(c));
        }
        for c in &childes {
            root.add_child(c);
        }

        crawl_tree(&root, &Default::default(), &mut result);
        let mut expect = Vec::new();
        for c in childes {
            expect.push(c.clone());
        }
        assert_eq!(expect, result);
    }
}
