use std::process::Command;

fn main() {
    Command::new("elm").args(&["make", "src/Main.elm", "--optimize", "--output=elm.js"])
                       .status().unwrap();
}