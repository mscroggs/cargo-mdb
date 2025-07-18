use std::process::Command;

const MIN_VERSION: [i32; 3] = [1, 0, 4];
const MDB_PIP_INSTALL: &str = "git+https://github.com/mscroggs/mdb.git";

/// Check if a version number is less than or equal to another version number
fn version_leq(v0: &[i32], v1: &[i32]) -> bool {
    if v0.is_empty() {
        true
    } else if v1.is_empty() || v0[0] > v1[0] {
        false
    } else if v0[0] < v1[0] {
        true
    } else {
        version_leq(&v0[1..], &v1[1..])
    }
}

/// Format version as string
fn version_str(v: &[i32]) -> String {
    v.iter()
        .map(|i| format!("{i}"))
        .collect::<Vec<_>>()
        .join(".")
}

fn main() {
    #[cfg(target_os = "windows")]
    let mut shell = Command::new("cmd /C");
    #[cfg(target_os = "windows")]
    shell.arg("/C");

    #[cfg(not(target_os = "windows"))]
    let mut shell = Command::new("sh");
    #[cfg(not(target_os = "windows"))]
    shell.arg("-c");
    shell.arg("mdb version");

    let output = shell
        .output()
        .expect("Error running mdb. Try installing with `pip install {MDB_PIP_INSTALL}`.");

    if !output.status.success() {
        panic!("Error running mdb. Try installing with `pip install {MDB_PIP_INSTALL}`.");
    }

    // TODO: update min version to [1, 0, 5] once mdb has next release
    let version = str::from_utf8(&output.stdout)
        .expect("")
        .replace("\n", "")
        .split(".")
        .map(|i| i.parse::<i32>().expect(""))
        .collect::<Vec<_>>();
    if !version_leq(&MIN_VERSION, &version) {
        panic!(
            "Found mdb. version {}\nmdb version must be {} or higher. Try upgrading with `pip install --upgrade {MDB_PIP_INSTALL}`.",
            version_str(&version),
            version_str(&MIN_VERSION),
        );
    }
}
