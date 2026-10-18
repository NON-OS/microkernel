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

//! The code rules the driver's tree is held to: a mod.rs declares modules
//! and re-exports them, nothing else (plan/41).

use std::fs;
use std::path::Path;

fn mod_files(dir: &Path, out: &mut Vec<std::path::PathBuf>) {
    for entry in fs::read_dir(dir).expect("readable").flatten() {
        let path = entry.path();
        if path.is_dir() {
            mod_files(&path, out);
        } else if path.file_name().is_some_and(|n| n == "mod.rs") {
            out.push(path);
        }
    }
}

#[test]
fn every_mod_rs_holds_declarations_only() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../capsule_driver_rtsx/src");
    let mut files = Vec::new();
    mod_files(&root, &mut files);
    assert!(files.len() >= 12, "{files:?}");
    let body = ["fn ", "struct ", "enum ", "impl ", "const ", "static ", "let "];
    for file in files {
        let text = fs::read_to_string(&file).expect("readable");
        for line in text.lines().map(str::trim).filter(|l| !l.starts_with("//")) {
            assert!(!body.iter().any(|b| line.contains(b)), "{}: {line}", file.display());
        }
    }
}
