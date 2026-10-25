// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! No caller of a flight stores into its keys itself: a plain store is one
//! the optimiser may drop, and `wipe_private()` is the one way they go. Every
//! userland source outside nonos_tls is read, so a fourth caller cannot come
//! back. nonos_vt is left out by name: its `private` is a CSI marker byte.

use std::path::Path;

use super::no_other_store_rule::stores;

const SKIP: &[&str] = &["nonos_tls", "tls_proofs", "nonos_vt", "target"];

fn walk(dir: &Path, found: &mut Vec<String>) {
    for e in std::fs::read_dir(dir).unwrap().flatten() {
        let p = e.path();
        let name = e.file_name().to_string_lossy().into_owned();
        if p.is_dir() {
            if !SKIP.contains(&name.as_str()) && !name.starts_with('.') {
                walk(&p, found);
            }
        } else if name.ends_with(".rs") {
            let text = std::fs::read_to_string(&p).unwrap_or_default();
            for (n, l) in text.lines().enumerate() {
                if stores(l) {
                    found.push(format!("{}:{}: {}", p.display(), n + 1, l.trim()));
                }
            }
        }
    }
}

#[test]
fn no_userland_file_outside_nonos_tls_stores_into_a_flights_keys() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let mut found = Vec::new();
    walk(&root, &mut found);
    assert!(found.is_empty(), "{found:#?}");
}
