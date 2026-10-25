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

//! vfs never calls the policy service. The policy service's keeper writes
//! the settings record through vfs and waits for the reply; a call back
//! would wait on a server that is waiting on vfs.

use std::fs;
use std::path::Path;

const VFS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../capsule_vfs");

fn sources(dir: &Path, out: &mut Vec<(String, String)>) {
    for entry in fs::read_dir(dir).expect("vfs sources") {
        let path = entry.expect("entry").path();
        if path.is_dir() {
            sources(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            let text = fs::read_to_string(&path).expect("read");
            out.push((path.display().to_string(), text));
        }
    }
}

#[test]
fn vfs_does_not_depend_on_the_policy_client() {
    let manifest = fs::read_to_string(format!("{VFS}/Cargo.toml")).expect("manifest");
    assert!(!manifest.contains("policy_client"), "capsule_vfs/Cargo.toml");
}

#[test]
fn no_vfs_source_names_the_policy_service() {
    let mut all = Vec::new();
    sources(&Path::new(VFS).join("src"), &mut all);
    assert!(all.len() > 20, "vfs sources not found");
    for (path, text) in all {
        for word in ["nonos_policy_client", "POLICY_SERVICE", "b\"policy\""] {
            assert!(!text.contains(word), "{path} names {word}");
        }
    }
}

#[test]
fn the_persist_gate_asks_the_kernel() {
    let gate = format!("{VFS}/src/server/handlers/persist_gate.rs");
    let text = fs::read_to_string(gate).expect("persist_gate.rs");
    let body = "fn may_persist() -> bool {\n    mk_keeping()\n}";
    assert!(text.contains(body), "persist_gate no longer asks the kernel alone");
}
