use std::{error::Error, fmt::Display, path::Path};

pub fn path_exists(path: &str) -> bool {
    Path::exists(Path::new(path))
}

pub fn drive_exists(drive: char) -> bool {
    let drive_name = drive.to_string() + ":";
    path_exists(&drive_name)
}

#[derive(Debug)]
pub enum DirError {
    LastDepth,
    AccessDenied(String),
}

impl Display for DirError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", &self)
    }
}
impl Error for DirError {}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn system_drive_exists_test() {
        assert_eq!(true, drive_exists('C'))
    }

    #[test]
    fn ilegal_drive_test() {
        assert_eq!(false, drive_exists('#'));
    }

    #[test]
    fn path_exists_test() {
        assert_eq!(true, path_exists("C:\\Users"));
    }

    #[test]
    fn path_not_exists_test() {
        assert_eq!(false, path_exists("#C:\\Users"));
    }
}
