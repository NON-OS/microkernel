use std::collections::HashSet;
use std::path::{Path, PathBuf};

use super::patterns::line_violation;
use super::roots::{is_rust, skip};
use super::test_only;

pub fn scan_path(
    path: &Path,
    tests: &HashSet<PathBuf>,
    out: &mut Vec<String>,
) -> std::io::Result<()> {
    if skip(path) || !is_rust(path) || tests.contains(path) {
        return Ok(());
    }
    let text = std::fs::read_to_string(path)?;
    let in_test = test_only::lines(&text);
    for (idx, line) in text.lines().enumerate() {
        if in_test[idx] {
            continue;
        }
        if let Some(kind) = line_violation(line) {
            out.push(format!("{}:{}:{kind}", path.display(), idx + 1));
        }
    }
    Ok(())
}

pub fn walk_dir(
    dir: &Path,
    tests: &HashSet<PathBuf>,
    out: &mut Vec<String>,
) -> std::io::Result<()> {
    if !dir.exists() {
        return Ok(());
    }
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            walk_dir(&path, tests, out)?;
        } else {
            scan_path(&path, tests, out)?;
        }
    }
    Ok(())
}
