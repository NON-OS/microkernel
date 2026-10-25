// NONOS Operating System (AGPL-3.0-or-later)
//! What the kernel's source names, counted per file, the way the static
//! check's baselines count it: every line, comments included.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

/// A line that gates on x86_64: `target_arch = "x86_64"`, however spaced.
pub fn names_x86_cfg(line: &str) -> bool {
    let tight: String = line.chars().filter(|c| !c.is_whitespace()).collect();
    tight.contains("target_arch=\"x86_64\"")
}

/// A line that reaches into the x86_64 tree by path.
pub fn names_x86_path(line: &str) -> bool {
    const PATH: &str = "crate::arch::x86_64";
    line.match_indices(PATH).any(|(at, _)| {
        let rest = &line[at + PATH.len()..];
        !rest.starts_with(|c: char| c.is_ascii_alphanumeric() || c == '_')
    })
}

/// Every `.rs` file under `root/src` but outside `root/src/arch`, keyed by
/// its path from `root`, with how many lines `hit` takes; files it takes
/// none of are left out. An unreadable directory or file is an error, never
/// a file with nothing in it.
pub fn count(root: &Path, hit: fn(&str) -> bool) -> std::io::Result<BTreeMap<String, usize>> {
    let mut out = BTreeMap::new();
    walk(root, &root.join("src"), hit, &mut out)?;
    Ok(out)
}

fn walk(
    root: &Path,
    dir: &Path,
    hit: fn(&str) -> bool,
    out: &mut BTreeMap<String, usize>,
) -> std::io::Result<()> {
    if dir == root.join("src").join("arch") {
        return Ok(());
    }
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            walk(root, &path, hit, out)?;
        } else if path.extension().is_some_and(|e| e == "rs") {
            let n = fs::read_to_string(&path)?.lines().filter(|l| hit(l)).count();
            if n > 0 {
                let key = path.strip_prefix(root).unwrap_or(&path);
                out.insert(key.to_string_lossy().replace('\\', "/"), n);
            }
        }
    }
    Ok(())
}
