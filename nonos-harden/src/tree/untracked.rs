// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Every file on disk the receipt's commit does not hold, found by walking
//! the tree without following a link. Not walked: `.git`, `.keys` and
//! `nonos-bootloader/keys` (git's own store, and key stores never opened or
//! listed), and every directory named `target` or `__pycache__`: build
//! outputs the verifiers this command starts do not read (it points cargo's
//! target directory and Python's bytecode cache at its own scratch space).

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

const SKIP_AT_ROOT: [&str; 3] = [".git", ".keys", "nonos-bootloader/keys"];
const SKIP_ANYWHERE: [&str; 2] = ["target", "__pycache__"];
const MAX_ENTRIES: usize = 4_000_000;

pub fn untracked(tree: &Path, tracked: &BTreeSet<&str>) -> Result<Vec<String>, String> {
    let (mut out, mut seen, mut todo) = (Vec::new(), 0usize, vec![String::new()]);
    while let Some(dir) = todo.pop() {
        let at = if dir.is_empty() { tree.to_path_buf() } else { tree.join(&dir) };
        let entries = fs::read_dir(&at).map_err(|e| format!("{}: {e}", at.display()))?;
        for e in entries {
            seen += 1;
            if seen > MAX_ENTRIES {
                return Err(format!("more than {MAX_ENTRIES} entries in the tree"));
            }
            let e = e.map_err(|e| e.to_string())?;
            let Some(name) = e.file_name().to_str().map(String::from) else {
                out.push(format!("{dir}/(a name that is not UTF-8)"));
                continue;
            };
            let rel = if dir.is_empty() { name.clone() } else { format!("{dir}/{name}") };
            let kind = e.file_type().map_err(|e| e.to_string())?;
            let skip =
                SKIP_ANYWHERE.contains(&name.as_str()) || SKIP_AT_ROOT.contains(&rel.as_str());
            match (kind.is_dir(), skip) {
                (true, true) => {}
                (true, false) => todo.push(rel),
                (false, _) if !tracked.contains(rel.as_str()) => out.push(rel),
                (false, _) => {}
            }
        }
    }
    out.sort();
    Ok(out)
}
