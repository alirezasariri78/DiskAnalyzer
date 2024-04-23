use std::ops::Deref;

use clap::*;

const DEPTH_ID: &str = "drive";
const DRIVES_ID: &str = "drives";
const PATH_ID: &str = "path";
const DIAGRAM_ID: &str = "diagram";
#[derive(Debug, Clone)]
pub enum DiagramType {
    tree,
    bar,
}

#[derive(Debug, Clone)]
pub struct CommandArgs {
    pub drive: Option<Vec<String>>,
    pub depth: usize,
    pub path: Option<String>,
    pub diagram: DiagramType,
}

impl DiagramType {
    fn from_string(input: &str) -> Self {
        match input {
            "bar" => DiagramType::bar,
            _ => DiagramType::tree,
        }
    }
}

impl CommandArgs {
    fn from_clap_args(args: ArgMatches) -> CommandArgs {
        let d = match args.get_one::<String>(DRIVES_ID) {
            Some(drive) => Some(
                drive
                    .split(' ')
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>(),
            ),
            None => None,
        };
        let path_arg = args.get_one::<String>(PATH_ID);
        let path = match path_arg {
            Some(p) => Some(p.to_string()),
            None => None,
        };

        let default_diagram = &String::from("tree");
        let diagram_arg: &String = args.get_one(DIAGRAM_ID).unwrap_or(default_diagram);
        let default_depth = &String::from("0");
        let depth_arg = args.get_one::<String>(DEPTH_ID).unwrap_or(default_depth);
        let depth = depth_arg.clone().parse().unwrap_or(0 as usize);
        CommandArgs {
            depth: depth,
            diagram: DiagramType::from_string(diagram_arg),
            drive: d,
            path,
        }
    }
}

pub fn get_args() -> CommandArgs {
    let args = command!()
        .arg(
            Arg::new(DEPTH_ID)
                .long("depth")
                .alias("dp")
                .alias("level")
                .help("how many level of inner directories should it scan"),
        )
        .arg(
            Arg::new(DIAGRAM_ID)
                .alias("Diagram")
                .long("diagram")
                .help("Set Diagram Types : tree , bar"),
        );

    let matched_result = handle_path_arg(args)
        .about("get information about size of folders in each drive")
        .get_matches();

    CommandArgs::from_clap_args(matched_result)
}

fn handle_path_arg(args: Command) -> Command {
    if cfg!(target_os = "macos|linux") {
        return args.arg(
            Arg::new(PATH_ID)
                .short('p')
                .alias("pt")
                .alias("pth")
                .long("path")
                .help("analyze give path"),
        );
    }

    args.arg(
        Arg::new(DRIVES_ID)
            .short('d')
            .long("drive")
            .help("which drive to scan.\nsplit with space.\n scan all drives if not set")
            .conflicts_with(PATH_ID),
    )
    .arg(
        Arg::new(PATH_ID)
            .short('p')
            .alias("pt")
            .alias("pth")
            .long("path")
            .conflicts_with(DRIVES_ID)
            .help("analyze give path"),
    )
}
