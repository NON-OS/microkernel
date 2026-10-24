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

//! proof_io runs only in an image that grants Debug.
//!
//! Its proof is a serial line, and its third step (MkDebug at a bad pointer,
//! expecting EFAULT) is refused for the capability first where Debug is not
//! granted, so in an image without `capsule-serial-debug` it exits 3: a false
//! proof failure on every boot. Every place the kernel spawns it must be
//! compiled only beside that feature. This walks the kernel's source for each
//! call and reads the cfg on the function around it.

use std::fs;
use std::path::{Path, PathBuf};

const CALL: &str = "spawn_proof_io_capsule()";

fn sources(dir: &Path, out: &mut Vec<PathBuf>) {
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
fn enclosing_cfg(lines: &[&str], at: usize) -> String {
    let f = (0..=at).rev().find(|&i| lines[i].trim_start().starts_with("fn ") || lines[i].contains(" fn "))
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
fn needs_serial_debug(cfg: &str) -> bool {
    let named = cfg.contains("feature = \"capsule-serial-debug\"");
    named && !cfg.contains("not(")
}

fn call_sites() -> Vec<(PathBuf, usize, String)> {
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

#[test]
fn every_proof_io_spawn_is_compiled_only_beside_serial_debug() {
    let sites = call_sites();
    assert!(!sites.is_empty(), "the kernel spawns proof_io somewhere; the walk found no call");
    for (file, line, cfg) in sites {
        assert!(needs_serial_debug(&cfg), "{}:{line} spawns proof_io under {cfg:?}", file.display());
    }
}

#[test]
fn the_reading_of_a_cfg_is_right() {
    assert!(needs_serial_debug("#[cfg(feature = \"capsule-serial-debug\")]"));
    assert!(needs_serial_debug("#[cfg(all(feature = \"nonos-user-entry-proof\", feature = \"capsule-serial-debug\"))]"));
    assert!(!needs_serial_debug(""));
    assert!(!needs_serial_debug("#[cfg(feature = \"nonos-user-entry-proof\")]"));
    assert!(!needs_serial_debug("#[cfg(not(feature = \"capsule-serial-debug\"))]"));
}
