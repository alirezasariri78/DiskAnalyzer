pub fn thousends_seperator(mut number: u64) -> String {
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
            result.insert_str(0, ",");
        }
    }

    result
}
