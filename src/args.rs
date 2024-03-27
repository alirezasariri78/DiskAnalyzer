use clap::*;

pub fn get_args() {
    let matched_result = command!()
        .arg(
            Arg::new("drive")
                .short('d')
                .long("drive")
                .help("which drive to scan.\nsplit with space.\n scan all drives if not set"),
        )
        .arg(
            Arg::new("depth")
                .long("depth")
                .alias("dp")
                .alias("level")
                .help("how many level of inner directories should it scan"),
        )
        .arg(
            Arg::new("threads")
                .short('t')
                .long("threads")
                .alias("thread")
                .alias("tr")
                .help("how many threads should it use for scanning.\n defulat to 2"),
        )
        .about("get information about size of folders in each drive")
        .get_matches();

    println!(
        "drive is :{} and depth is : {} threads is :{}",
        matched_result.get_one::<String>("drive").unwrap(),
        matched_result.get_one::<String>("depth").unwrap(),
        matched_result.get_one::<String>("threads").unwrap()
    );
}
