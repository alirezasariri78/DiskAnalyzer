use crate::args::CommandArgs;
use crate::crawler::Node;
use cli_table::{format::Justify, Cell, CellStruct, Style, Table, TableStruct};
use rs_abbreviation_number::NumericAbbreviate;
use std::{ops::Deref, sync::Arc};

pub fn create_table_diagram(tree: &Arc<Node>, args: &CommandArgs) -> TableStruct {
    let mut data: Vec<Vec<CellStruct>> = Vec::new();
    crawl_tree(tree, args, &mut data);
    data.table()
        .title(vec![
            "Name".cell().justify(Justify::Center).bold(true),
            "Size".cell().justify(Justify::Center).bold(true),
        ])
        .bold(true)
}

fn crawl_tree(tree: &Arc<Node>, args: &CommandArgs, result: &mut Vec<Vec<CellStruct>>) {
    if tree.get_depth().get() == args.depth && args.depth != 0 {
        return;
    }
    let childrens = tree.get_childes().borrow();
    let deref = childrens.deref();
    for child in deref {
        let node = child.deref();
        let size = format!(
            "{}iB ({} bytes)",
            node.get_size().get().abbreviate_number(),
            node.get_size().get()
        );
        result.push(vec![
            node.get_name().cell().justify(Justify::Center),
            size.cell().justify(Justify::Center),
        ]);
        crawl_tree(child, args, result);
    }
}
