use std::process::Command;

fn main() {
    let exitcode = Command::new("elm").args(&["make", "src/Main.elm", "--debug", "--output=elm.js"])
                       .status().unwrap();
    assert!(exitcode.success());
}