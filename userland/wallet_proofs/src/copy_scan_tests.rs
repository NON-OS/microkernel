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

//! No wallet, keyring or shield code writes the clipboard but the one gate.

use std::path::Path;

fn scan(dir: &Path, hits: &mut Vec<String>) {
    for entry in std::fs::read_dir(dir).expect("source dir") {
        let path = entry.expect("entry").path();
        if path.is_dir() {
            scan(&path, hits);
        } else if path.extension().is_some_and(|e| e == "rs") {
            let text = std::fs::read_to_string(&path).expect("source");
            if text.contains("clipboard_copy") || text.contains("clipboard::copy") {
                hits.push(path.display().to_string());
            }
        }
    }
}

#[test]
fn only_the_gate_writes_the_clipboard() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let mut hits = Vec::new();
    for capsule in ["capsule_wallet_nonos", "capsule_keyring", "capsule_shield"] {
        scan(&root.join(capsule).join("src"), &mut hits);
    }
    let gate = Path::new("capsule_wallet_nonos/src/wallet/copy/mod.rs");
    assert_eq!(hits.len(), 1, "clipboard writers: {hits:?}");
    assert!(Path::new(&hits[0]).ends_with(gate), "clipboard writers: {hits:?}");
}
