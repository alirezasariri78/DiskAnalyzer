use clap::*;

const DEPTH_ID: &str = "drive";
const THREADS_ID: &str = "threads";
const DRIVES_ID: &str = "drives";

#[derive(Debug, Clone)]
pub struct CommandArgs {
    pub drive: Vec<String>,
    pub depth: usize,
    pub threads: usize,
}

impl CommandArgs {
    fn from_clap_args(args: ArgMatches) -> CommandArgs {
        CommandArgs {
            depth: *args.get_one::<usize>(DEPTH_ID).unwrap_or(&5),
            threads: *args.get_one::<usize>(THREADS_ID).unwrap_or(&2),
            drive: args
                .get_one::<String>(DRIVES_ID)
                .unwrap_or(&String::new())
                .split(' ')
                .map(|x| x.to_string())
                .collect(),
        }
    }
}

pub fn get_args() -> CommandArgs {
    let matched_result = command!()
        .arg(
            Arg::new(DRIVES_ID)
                .short('d')
                .long("drive")
                .help("which drive to scan.\nsplit with space.\n scan all drives if not set"),
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
        .about("get information about size of folders in each drive")
        .get_matches();

    CommandArgs::from_clap_args(matched_result)
}
