pub fn thousends_seperator(i: u64) -> String {
    let mut s = String::new();
    let i_str = i.to_string();
    let a = i_str.chars().rev().enumerate();
    for (idx, val) in a {
        if idx != 0 && idx % 3 == 0 {
            s.insert(0, ',');
        }
        s.insert(0, val);
    }
    s
}

mod tests {

    #[test]
    fn thousends_seperator_test() {
        assert_eq!("1,000,000", super::thousends_seperator(1_000_000));
        assert_eq!("1", super::thousends_seperator(1));
        assert_eq!("1", super::thousends_seperator(001));
        assert_eq!("1", super::thousends_seperator(00001));
    }
}
