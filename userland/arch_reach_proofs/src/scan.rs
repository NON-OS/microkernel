// NONOS Operating System (AGPL-3.0-or-later)
//! What the kernel's source names, counted per file by the rules in
//! `rules`, the way the static check's baselines count: every line,
//! comments included.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

pub use crate::rules::{hits, names_x86_cfg, names_x86_path, Rule};

/// Every `.rs` file under `root/src` but outside `root/src/arch`, keyed by
/// its path from `root`, with how many lines `rule` takes; files it takes
/// none of are left out. An unreadable directory or file is an error, never
/// a file with nothing in it.
pub fn count(root: &Path, rule: Rule) -> std::io::Result<BTreeMap<String, usize>> {
    let mut out = BTreeMap::new();
    walk(root, &root.join("src"), rule, &mut out)?;
    Ok(out)
}

fn walk(
    root: &Path,
    dir: &Path,
    rule: Rule,
    out: &mut BTreeMap<String, usize>,
) -> std::io::Result<()> {
    if dir == root.join("src").join("arch") {
        return Ok(());
    }
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            walk(root, &path, rule, out)?;
        } else if path.extension().is_some_and(|e| e == "rs") {
            let n = hits(&fs::read_to_string(&path)?, rule);
            if n > 0 {
                let key = path.strip_prefix(root).unwrap_or(&path);
                out.insert(key.to_string_lossy().replace('\\', "/"), n);
            }
        }
    }
    Ok(())
}
