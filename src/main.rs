mod args;
mod crawler;

use args as arg;
fn main() {
    let arguments = arg::get_args();
    let tree = crawler::get_tree(&arguments);
    dbg!(tree);
}
