#[cfg(test)]
mod tests {
    use std::env;
    use std::process::Command;
    const MIDDLE_CHAR: &'static str = "├──";

    #[test]
    fn table_test() {
        let current_path = env::current_dir().expect("Failed To Get Current Dir");
        let program_path = current_path.join("target").join("debug").join("diska");
        let command = Command::new(program_path)
            .arg("-p")
            .arg(
                current_path
                    .to_str()
                    .expect("Failed To Pars Current Path To Str"),
            )
            .arg("--diagram")
            .arg("table")
            .output()
            .expect("Failed To Execute Command");

        assert!(command.status.success());
        assert!(String::from_utf8_lossy(&command.stdout).contains("+"));
    }

    #[test]
    fn tree_test() {
        let current_path = env::current_dir().expect("Failed To Get Current Dir");
        let program_path = current_path.join("target").join("debug").join("diska");
        let command = Command::new(program_path)
            .arg("-p")
            .arg(
                current_path
                    .to_str()
                    .expect("Failed To Pars Current Path To Str"),
            )
            .arg("--diagram")
            .arg("tree")
            .output()
            .expect("Failed To Execute Command");

        println!("{}", String::from_utf8_lossy(&command.stdout));
        assert!(command.status.success());
        assert!(String::from_utf8_lossy(&command.stdout).contains(MIDDLE_CHAR));
    }

    #[test]
    fn default_diagram_test() {
        let current_path = env::current_dir().expect("Failed To Get Current Dir");
        let program_path = current_path.join("target").join("debug").join("diska");
        let command = Command::new(program_path)
            .arg("-p")
            .arg(
                current_path
                    .to_str()
                    .expect("Failed To Pars Current Path To Str"),
            )
            .output()
            .expect("Failed To Execute Command");

        println!("{}", String::from_utf8_lossy(&command.stdout));
        assert!(command.status.success());
        assert!(String::from_utf8_lossy(&command.stdout).contains(MIDDLE_CHAR));
    }
}
