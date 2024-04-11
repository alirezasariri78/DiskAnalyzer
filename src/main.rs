mod args;
mod crawler;

use args as arg;
fn main() {
    let arguments = arg::get_args();
    let tree = crawler::get_tree(&arguments);
    let s = tree.get_size();
    println!("{}", format_number(s));
    dbg!(tree);
    let mut a = String::new();
    std::io::stdin().read_line(&mut a).unwrap();
}

fn format_number(mut number: u64) -> String {
    let mut result = String::new();
    let mut remainder = number % 1000;

    while remainder > 0 || number > 0 {
        if remainder > 0 {
            result.insert_str(0, format!("{:03}", remainder).as_str());
            remainder = 0;
        } else {
            let thousands = number % 1000;
            number /= 1000;

            if thousands > 0 {
                remainder = thousands;
            }
        }

        if number > 0 {
            result.insert_str(0, " ");
        }
    }

    result
}

mod tests {
    use super::*;
    #[test]
    fn custom_folder_test() {}
}
