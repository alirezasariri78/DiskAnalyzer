use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use piechart::{Chart, Color, Data};
use crate::crawler::{Node, NodeType};
use crate::args::CommandArgs;

pub fn create_media_type_diagram(tree: &Arc<Node>, args: &CommandArgs){
   let mut data:HashMap<NodeType,f32>=HashMap::from(
                                    [(NodeType::Video,0.0)
                                    ,(NodeType::Image,0.0)
                                    ,(NodeType::File,0.0)
                                    ,(NodeType::Audio,0.0)]);
    crawl_tree(tree, args, &mut data);
    export_to_diagram(&data);
}

fn export_to_diagram(data: &HashMap<NodeType,f32>) {    
    let result = vec![
        Data { label: NodeType::Video.to_string().into(), value:*data.get(&NodeType::Video).unwrap_or(&0.0), color: Some(Color::Blue.into()), fill: '•' },
        Data { label:  NodeType::Image.to_string().into(), value:*data.get(&NodeType::Image).unwrap_or(&0.0), color: Some(Color::Red.into()), fill: '▪' },
        Data { label:  NodeType::Audio.to_string().into(), value:*data.get(&NodeType::Audio).unwrap_or(&0.0), color: Some(Color::Yellow.into()), fill: '▴' },
        Data { label:  NodeType::File.to_string().into(), value: *data.get(&NodeType::File).unwrap_or(&0.0), color: Some(Color::Green.into()), fill: '▰' },
    ];


    Chart::new()
        .radius(9)
        .aspect_ratio(3)
        .legend(true)
        .draw(&result);
}

fn crawl_tree(tree: &Arc<Node>, args: &CommandArgs, result: &mut HashMap<NodeType,f32>) {
    if tree.get_depth().get() == args.depth && args.depth != 0 {
        return;
    }
    let childrens = tree.get_childes().borrow();
    let deref = childrens.deref();
    for child in deref {
        let node_type=child.get_node_type();
        if node_type!= &NodeType::Directory{
            result.entry(node_type.clone()).and_modify(|e|*e+=child.get_size().get() as f32);
        }else {
            crawl_tree(&child, args, result);
        }
    }
}
