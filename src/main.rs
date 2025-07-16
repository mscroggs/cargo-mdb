//! cargo-mdb
//!
//! Install using `cargo install cargo-mdb`
//!
//! Run using: `cargo mdb`
#![cfg_attr(feature = "strict", deny(warnings), deny(unused_crate_dependencies))]
#![warn(missing_docs)]

use std::env;
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

/// File to run
enum FileToRun {
    /// An example
    Example(String),
    /// A binary
    Bin(String),
}

fn main() {
    let input_args = env::args().collect::<Vec<_>>();
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
    for a in &build_args {
        build_command = format!("{build_command} {a}");
    }

    if run_command(&build_command).is_err() {
        panic!("Build step failed");
    }

    // Launch
    let target = match file {
        FileToRun::Example(e) => format!("./target/debug/examples/{e}"),
        FileToRun::Bin(e) => format!("./target/debug/{e}"),
    };
    let mut launch_command = format!("mdb launch -t {target} -b rust-gdb -n {nprocesses}");
    if let Some(p) = port {
        launch_command = format!("{launch_command} -p {p}");
    }
    let _ = run_command(&launch_command);
}
