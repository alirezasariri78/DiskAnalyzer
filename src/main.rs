mod args;
mod crawler;

use args as arg;
fn main() {
    let tree = crawler::get_tree();
    let arguments = arg::get_args();
    // dbg!(tree);
}
