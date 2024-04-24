use clap::*;

const DEPTH_ID: &str = "depth";
const DRIVES_ID: &str = "drives";
const PATH_ID: &str = "path";
const DIAGRAM_ID: &str = "diagram";
const SORT_ID: &str = "sort";
const SORTTYPE_ID: &str = "st";

trait ArgBuilder {
    fn parse_drive(&mut self) -> Self;
    fn parse_depth(&mut self) -> Self;
    fn parse_path(&mut self) -> Self;
    fn parse_diagram(&mut self) -> Self;
    fn parse_sort(&mut self) -> Self;
}

#[derive(Debug, Clone)]
pub enum DiagramType {
    Tree,
    Table,
}

#[derive(Debug, Clone)]
pub enum SortType {
    Size(bool),
    Name(bool),
    Default,
}

#[derive(Debug, Clone)]
pub struct CommandArgs {
    args: ArgMatches,
    pub drive: Option<Vec<String>>,
    pub depth: usize,
    pub path: Option<String>,
    pub diagram: DiagramType,
    pub sort: SortType,
}

impl Default for CommandArgs {
    fn default() -> Self {
        Self {
            args: Default::default(),
            drive: None,
            depth: 0,
            path: None,
            diagram: DiagramType::Tree,
            sort: SortType::Default,
        }
    }
}

impl ArgBuilder for CommandArgs {
    fn parse_drive(&mut self) -> Self {
        let mut drive = None;
        if cfg!(target_os = "windows") {
            drive = match self.args.get_one::<String>(DRIVES_ID) {
                Some(drive) => Some(
                    drive
                        .split(' ')
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>(),
                ),
                None => None,
            };
        }
        self.drive = drive;
        self.to_owned()
    }

    fn parse_depth(&mut self) -> Self {
        let default = &"0".to_string();
        let depth_arg = self.args.get_one::<String>(DEPTH_ID).unwrap_or(default);
        let depth = depth_arg.clone().parse().unwrap_or(0 as usize);
        self.depth = depth;
        self.to_owned()
    }

    fn parse_path(&mut self) -> Self {
        let path_arg = self.args.get_one::<String>(PATH_ID);
        let path = match path_arg {
            Some(p) => Some(p.to_string()),
            None => None,
        };
        self.path = path;
        self.to_owned()
    }

    fn parse_diagram(&mut self) -> Self {
        let default_diagram = &String::from("tree");
        let diagram_arg: &String = self.args.get_one(DIAGRAM_ID).unwrap_or(default_diagram);
        self.diagram = DiagramType::from_string(diagram_arg);
        self.to_owned()
    }

    fn parse_sort(&mut self) -> Self {
        match self.args.get_one::<String>(SORT_ID) {
            Some(sort_arg) => {
                let default_sort_type = &String::from("desc");
                let sort_type_arg: &String =
                    self.args.get_one(SORTTYPE_ID).unwrap_or(default_sort_type);

                self.sort = SortType::from_string(
                    sort_arg.as_str(),
                    sort_type_arg.to_lowercase() == "desc",
                );
            }
            None => self.sort = SortType::Default,
        }
        self.to_owned()
    }
}

impl DiagramType {
    fn from_string(input: &str) -> Self {
        match input.to_lowercase().as_str() {
            "table" => DiagramType::Table,
            "tree" => DiagramType::Tree,
            _ => DiagramType::Tree,
        }
    }
}

impl SortType {
    fn from_string(input: &str, desc: bool) -> Self {
        match input.to_lowercase().as_str() {
            "size" => SortType::Size(desc),
            "name" => SortType::Name(desc),
            _ => SortType::Default,
        }
    }
}

impl CommandArgs {
    fn new(args: ArgMatches) -> Self {
        CommandArgs {
            args,
            ..Default::default()
        }
    }

    fn from_clap_args(args: ArgMatches) -> CommandArgs {
        CommandArgs::new(args)
            .parse_depth()
            .parse_diagram()
            .parse_drive()
            .parse_path()
            .parse_sort()
    }
}

pub fn get_args() -> CommandArgs {
    let args = command!()
        .arg(
            Arg::new(DEPTH_ID)
                .ignore_case(true)
                .long("depth")
                .alias("dp")
                .alias("level")
                .help("how many level of inner directories should it scan"),
        )
        .arg(
            Arg::new(DIAGRAM_ID)
                .ignore_case(true)
                .long("diagram")
                .help("Set Diagram Types : tree , bar"),
        )
        .arg(
            Arg::new(SORT_ID)
                .ignore_case(true)
                .short('s')
                .alias("order")
                .long("sort")
                .help("Sort Folders Based On Given Value : size , name\n"),
        )
        .arg(
            Arg::new(SORTTYPE_ID)
                .help("Can Use By sort \nSort Folders:  Desc or Asc")
                .long("st")
                .ignore_case(true),
        );

    let matched_result = handle_path_arg(args)
        .about("get information about size of folders in each drive")
        .get_matches();

    CommandArgs::from_clap_args(matched_result)
}

fn handle_path_arg(args: Command) -> Command {
    if cfg!(target_os = "windows") {
        return handle_windows_path_arg(args);
    }
    handle_linux_path_arg(args)
}

fn handle_windows_path_arg(args: Command) -> Command {
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

fn handle_linux_path_arg(args: Command) -> Command {
    args.arg(
        Arg::new(PATH_ID)
            .short('p')
            .alias("pt")
            .alias("pth")
            .long("path")
            .help("analyze give path"),
    )
}
