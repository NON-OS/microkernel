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

//! The walk proof_io_spawn_tests reads the kernel by: every call that spawns
//! proof_io, and the cfg on the function around it.

use std::fs;
use std::path::{Path, PathBuf};

const CALL: &str = "spawn_proof_io_capsule()";

pub(super) fn sources(dir: &Path, out: &mut Vec<PathBuf>) {
    for e in fs::read_dir(dir).expect("readable source tree") {
        let p = e.expect("entry").path();
        if p.is_dir() {
            sources(&p, out);
        } else if p.extension().is_some_and(|x| x == "rs") {
            out.push(p);
        }
    }
}

/// The cfg lines directly above the `fn` that encloses line `at`.
pub(super) fn enclosing_cfg(lines: &[&str], at: usize) -> String {
    let f = (0..=at)
        .rev()
        .find(|&i| lines[i].trim_start().starts_with("fn ") || lines[i].contains(" fn "))
        .expect("a call inside a function");
    let mut cfg = String::new();
    let mut i = f;
    while i > 0 && lines[i - 1].trim_start().starts_with("#[") {
        i -= 1;
        cfg.insert_str(0, lines[i]);
    }
    cfg
}

/// Whether a cfg holds only where `capsule-serial-debug` is on: it names the
/// feature, and not under a `not(`.
pub(super) fn needs_serial_debug(cfg: &str) -> bool {
    let named = cfg.contains("feature = \"capsule-serial-debug\"");
    named && !cfg.contains("not(")
}

pub(super) fn call_sites() -> Vec<(PathBuf, usize, String)> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../src");
    let mut files = Vec::new();
    sources(&root, &mut files);
    let mut found = Vec::new();
    for f in files {
        let text = fs::read_to_string(&f).expect("utf-8 source");
        let lines: Vec<&str> = text.lines().collect();
        for (i, l) in lines.iter().enumerate() {
            let code = l.split("//").next().unwrap_or("");
            if code.contains(CALL) && !code.contains("fn spawn_proof_io_capsule") {
                found.push((f.clone(), i + 1, enclosing_cfg(&lines, i)));
            }
        }
    }
    found
}
