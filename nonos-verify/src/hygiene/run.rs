use std::path::Path;

use crate::report::{Report, Status};

use super::baseline;
use super::roots::root_dirs;
use super::scan::walk_dir;
use super::test_only;

pub fn run(root: &str) -> std::io::Result<Status> {
    let mut rpt = Report::new("hygiene", true);
    let out = Path::new(root).join("hygiene");
    std::fs::create_dir_all(&out)?;
    let mut violations = Vec::new();
    let tests = test_only::files(root_dirs());
    for dir in root_dirs() {
        walk_dir(Path::new(dir), &tests, &mut violations)?;
    }
    // The sealed tree's markers are each named, with why, in the baseline:
    // a new one fails, and so does a row the tree no longer matches.
    let findings = match std::fs::read_to_string(baseline::FILE) {
        Ok(text) => match baseline::parse(&text) {
            Ok(rows) => baseline::compare(&rows, &violations),
            Err(e) => vec![format!("{}: {e}", baseline::FILE)],
        },
        Err(e) => vec![format!("{}: {e}", baseline::FILE)],
    };
    let mut log = if violations.is_empty() {
        "no production panic or stub markers\n".to_string()
    } else {
        format!("{} marker(s), each to match a row of {}:\n", violations.len(), baseline::FILE)
            + &violations.join("\n")
            + "\n"
    };
    if !findings.is_empty() {
        log += &format!("\n{} finding(s):\n{}\n", findings.len(), findings.join("\n"));
        eprintln!("production-source-hygiene:\n  {}", findings.join("\n  "));
    }
    std::fs::write(out.join("production-source-hygiene.txt"), log)?;
    rpt.check(
        "production-source-hygiene",
        if findings.is_empty() { Status::Pass } else { Status::Fail },
        format!(
            "{} marker(s) in the sealed tree's baseline, {} finding(s)",
            violations.len(),
            findings.len()
        ),
    );
    rpt.finish(root)
}
