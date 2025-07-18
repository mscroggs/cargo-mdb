//! cargo-mdb
//!
//! Install using `cargo install cargo-mdb`
//!
//! Run using: `cargo mdb`
#![cfg_attr(feature = "strict", deny(warnings), deny(unused_crate_dependencies))]
#![warn(missing_docs)]

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Run a command
fn run_command(command: &str) -> Result<(), &str> {
    #[cfg(target_os = "windows")]
    let mut shell = Command::new("cmd /C");
    #[cfg(target_os = "windows")]
    shell.arg("/C");

    #[cfg(not(target_os = "windows"))]
    let mut shell = Command::new("sh");
    #[cfg(not(target_os = "windows"))]
    shell.arg("-c");
    shell.arg(command);

    let mut child = shell.spawn().expect("Error initialising command");
    match child.wait().expect("Error running command").code() {
        Some(0) => Ok(()),
        _ => Err("command run failed"),
    }
}

/// Find directory containing Cargo.toml
pub fn find_cargo_toml() -> PathBuf {
    let mut dir = env::current_dir().expect("Cannot find current dir");
    while !join(&dir, "Cargo.toml").exists() {
        dir = dir.parent().expect("Cannot find Cargo.toml").to_path_buf();
    }
    dir
}

/// Join a directory and a file name
pub fn join(part1: &impl AsRef<Path>, part2: &str) -> PathBuf {
    let mut out = PathBuf::from(part1.as_ref());
    out.push(part2);
    out
}

/// File to run
enum FileToRun {
    /// An example
    Example(String),
    /// A binary
    Bin(String),
}

fn main() {
    let input_args = env::args().collect::<Vec<_>>();

    let root_dir = find_cargo_toml();

    assert_eq!(input_args[1], "mdb");
    let mut input_args = input_args[2..].iter();

    let mut port = None;
    let mut nprocesses = 2;
    let mut file = None;

    let mut build_args = vec![];
    while let Some(arg) = input_args.next() {
        match arg.as_ref() {
            "-n" => {
                nprocesses = input_args
                    .next()
                    .expect("-n must be followed by a value")
                    .parse::<usize>()
                    .expect("Value following -n must be an integer");
            }
            "-p" => {
                port = Some(
                    input_args
                        .next()
                        .expect("-p must be followed by a value")
                        .parse::<usize>()
                        .expect("Value following -p must be an integer"),
                );
            }
            "--release" => {
                // TODO: warning here rather than panic
                panic!("mdb will not work well in release mode");
            }
            "--example" => {
                if file.is_some() {
                    panic!("Cannot run two files");
                }
                file = Some(FileToRun::Example(String::from(
                    input_args
                        .next()
                        .expect("--example must be followed by a value"),
                )));
            }
            "--bin" => {
                if file.is_some() {
                    panic!("Cannot run two files");
                }
                file = Some(FileToRun::Bin(String::from(
                    input_args
                        .next()
                        .expect("--bin must be followed by a value"),
                )));
            }
            _ => {
                build_args.push(arg);
            }
        }
    }
    let file = file.expect("Must include a file to run");

    // Build
    let mut build_command = String::from("cargo build");
    build_command.push(' ');
    for arg in &build_args {
        build_command.push(' ');
        build_command.push_str(arg);
    }

    if run_command(&build_command).is_err() {
        panic!("Build step failed");
    }

    // Launch
    let target = join(
        &root_dir,
        &match file {
            FileToRun::Example(e) => format!("target/debug/examples/{e}"),
            FileToRun::Bin(e) => format!("target/debug/{e}"),
        },
    );
    let mut launch_command = format!(
        "mdb launch -t {} -b rust-gdb -n {nprocesses}",
        target.display()
    );
    if let Some(p) = port {
        launch_command.push_str(&format!(" -p {p}"));
    }
    let _ = run_command(&launch_command);
}
