mod args;
mod crawler;
mod diagram;
mod util;

use args as arg;
use diagram::*;
fn main() {
    let arguments = arg::get_args();
    let tree = crawler::get_tree(&arguments);
    show_diagram(&tree, &arguments);

    let mut read = String::new();
    _ = std::io::stdin().read_line(&mut read);
}
