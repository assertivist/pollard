use std::process::Command;

fn main() {
    Command::new("elm").args(&["make", "--optimize", "src/Main.elm"])
                       .status().unwrap();
}