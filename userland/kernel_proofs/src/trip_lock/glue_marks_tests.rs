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

//! G7 (L81b): every listed spin lock is marked where it is taken, so a rest
//! taken while one is held is said. Listed so far: the transport lock's
//! HOLDERS and the trip locks' rows; later steps add theirs with their fix.
//! The drop's `rows.lock().give(..)` is exempt: a temporary held for one
//! call that never rests.

use std::path::Path;

use crate::proofs::proofs_8::transport_takeover::text::code;

const LISTED: [&str; 2] = ["HOLDERS.lock()", "self.rows.lock()"];

fn walk(dir: &Path, out: &mut Vec<(String, String)>) {
    for entry in std::fs::read_dir(dir).expect("src readable") {
        let path = entry.expect("entry").path();
        if path.is_dir() {
            walk(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push((path.display().to_string(), std::fs::read_to_string(&path).expect("read")));
        }
    }
}

/// The code of the block that holds position `at`, up to `at`.
fn block_before(c: &str, at: usize) -> &str {
    let mut depth = 0i32;
    for (i, b) in c[..at].bytes().enumerate().rev() {
        match b {
            b'}' => depth += 1,
            b'{' if depth == 0 => return &c[i..at],
            b'{' => depth -= 1,
            _ => {}
        }
    }
    &c[..at]
}

#[test]
fn g7_every_listed_spin_lock_is_marked_where_it_is_taken() {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../src");
    let mut files = Vec::new();
    walk(&src, &mut files);
    let mut seen = 0;
    for (path, text) in files {
        let c = code(&text);
        for lock in LISTED {
            let lock = code(lock);
            for (at, _) in c.match_indices(&lock) {
                if c[at + lock.len()..].starts_with(".give(") {
                    continue;
                }
                seen += 1;
                assert!(block_before(&c, at).contains("quiet()"), "{path}: {lock} unmarked");
            }
        }
    }
    assert_eq!(seen, 4, "the listed takes moved: HOLDERS twice, the look's rows, forget's rows");
}
