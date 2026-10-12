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

//! The code rules, held over the driver's whole source: no file past 75
//! lines, no run of blank lines, and a SAFETY comment on every unsafe block.

use std::path::{Path, PathBuf};

fn sources(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).expect("driver source") {
        let path = entry.expect("entry").path();
        if path.is_dir() {
            sources(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

fn driver() -> Vec<(PathBuf, String)> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../capsule_driver_virtio_net/src");
    let mut files = Vec::new();
    sources(&root, &mut files);
    let read = |p: PathBuf| (p.strip_prefix(&root).unwrap_or(&p).into(), read(&p));
    files.into_iter().map(read).collect()
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).expect("read")
}

#[test]
fn no_driver_file_is_longer_than_75_lines_or_has_blank_runs() {
    for (path, text) in driver() {
        let lines: Vec<&str> = text.lines().collect();
        assert!(lines.len() <= 75, "{} has {} lines", path.display(), lines.len());
        let run = lines.windows(2).position(|w| w[0].trim().is_empty() && w[1].trim().is_empty());
        assert!(run.is_none(), "{} has blank lines at {}", path.display(), run.unwrap_or(0) + 1);
    }
}

#[test]
fn every_unsafe_block_says_why_it_holds() {
    for (path, text) in driver() {
        let lines: Vec<&str> = text.lines().collect();
        for (i, line) in lines.iter().enumerate() {
            let code = line.split("//").next().unwrap_or("");
            if !(code.contains("unsafe {") || code.contains("unsafe impl")) {
                continue;
            }
            let above = lines[..i].iter().rev().take_while(|l| l.trim_start().starts_with("//"));
            let said = line.contains("SAFETY:") || above.clone().any(|l| l.contains("SAFETY:"));
            assert!(said, "{}:{} has no SAFETY comment", path.display(), i + 1);
        }
    }
}
