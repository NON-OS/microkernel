use std::path::Path;

use crate::report::{Report, Status};

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
    let log = if violations.is_empty() {
        "no production panic or stub markers\n".to_string()
    } else {
        violations.join("\n") + "\n"
    };
    std::fs::write(out.join("production-source-hygiene.txt"), log)?;
    rpt.check(
        "production-source-hygiene",
        if violations.is_empty() { Status::Pass } else { Status::Fail },
        format!("{} violation(s)", violations.len()),
    );
    rpt.finish(root)
}
