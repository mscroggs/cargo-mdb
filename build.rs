use std::process::Command;

fn main() {
    Command::new("pip")
        .args(&["install", "git+https://github.com/TomMelt/mdb@rusty"])
        .status()
        .unwrap();
}
