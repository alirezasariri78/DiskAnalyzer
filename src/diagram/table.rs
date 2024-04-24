use crate::crawler::Node;
use crate::{
    args::{CommandArgs, SortType},
    util::thousends_seperator,
};
use cli_table::{format::Justify, Cell, CellStruct, Style, Table, TableStruct};
use rs_abbreviation_number::NumericAbbreviate;
use std::{ops::Deref, sync::Arc};

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
        let size = format!(
            "{}iB ({} bytes)",
            size.abbreviate_number(),
            thousends_seperator(size)
        );
        result.push(vec![
            name.cell().justify(Justify::Center),
            size.cell().justify(Justify::Center),
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
            if t {
                data.sort_by(|a, b| a.0.cmp(b.0));
            } else {
                data.sort_by(|a, b| b.0.cmp(a.0));
            }
            data
        }
        SortType::Size(t) => {
            if t {
                data.sort_by(|a, b| a.1.cmp(&b.1));
            } else {
                data.sort_by(|a, b| b.1.cmp(&a.1));
            }
            data
        }
        _ => data,
    }
}
