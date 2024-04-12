use clap::*;

const DEPTH_ID: &str = "drive";
const THREADS_ID: &str = "threads";
const DRIVES_ID: &str = "drives";
const PATH_ID: &str = "path";

#[derive(Debug, Clone)]
pub struct CommandArgs {
    pub drive: Option<Vec<String>>,
    pub depth: usize,
    pub threads: usize,
    pub path: Option<String>,
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
        CommandArgs {
            depth: *args.get_one::<usize>(DEPTH_ID).unwrap_or(&0),
            threads: *args.get_one::<usize>(THREADS_ID).unwrap_or(&2),
            drive: d,
            path,
        }
    }
}

pub fn get_args() -> CommandArgs {
    let matched_result = command!()
        .arg(
            Arg::new(DRIVES_ID)
                .short('d')
                .long("drive")
                .help("which drive to scan.\nsplit with space.\n scan all drives if not set")
                .conflicts_with(PATH_ID),
        )
        .arg(
            Arg::new(DEPTH_ID)
                .long("depth")
                .alias("dp")
                .alias("level")
                .help("how many level of inner directories should it scan"),
        )
        .arg(
            Arg::new(THREADS_ID)
                .short('t')
                .long("threads")
                .alias("thread")
                .alias("tr")
                .help("how many threads should it use for scanning.\n defulat to 2"),
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
        .about("get information about size of folders in each drive")
        .get_matches();

    CommandArgs::from_clap_args(matched_result)
}
