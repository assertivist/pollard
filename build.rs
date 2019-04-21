use std::process::Command;

fn main() {
    let exitcode = Command::new("elm").args(&["make", "src/Main.elm", "--optimize", "--output=elm.js"])
                       .status().unwrap();
    assert!(exitcode.success());
}