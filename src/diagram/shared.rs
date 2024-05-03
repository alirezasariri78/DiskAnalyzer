pub(crate) fn get_color_from_size(size: u64) -> &'static str {
    const GREEN_SIZE_BYTES: u64 = 1_024_000_000;
    const YELLOW_SIZE_BYTES: u64 = GREEN_SIZE_BYTES * 10;
    const YELLOW_START: u64 = GREEN_SIZE_BYTES + 1;
    match size {
        0..=GREEN_SIZE_BYTES => "Green",
        YELLOW_START..=YELLOW_SIZE_BYTES => "Yellow",
        _ => "Red",
    }
}

mod tests {

    use super::*;

    #[test]
    fn green_size_test() {
        assert_eq!("Green", get_color_from_size(0));
        assert_eq!("Green", get_color_from_size(453));
        assert_eq!("Green", get_color_from_size(1_024_000_000));
    }

    #[test]
    fn yellow_size_test() {
        assert_eq!("Yellow", get_color_from_size(1_024_000_001));
        assert_eq!("Yellow", get_color_from_size(1_024_439_300));
        assert_eq!("Yellow", get_color_from_size(1_024_000_0000));
    }

    #[test]
    fn red_size_test() {
        assert_eq!("Red", get_color_from_size(1_024_000_001_000));
        assert_eq!("Red", get_color_from_size(1_024_000_0001));
        assert_eq!("Red", get_color_from_size(999_999_999_999_999_999_9));
    }
}
