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

//! Every call into `process::caps`'s mutators, which take its MUTATE lock
//! and, under it, the process table, the paging manager and token locks.
//! Each site below was read for what it holds when it calls; none holds the
//! process table, the paging manager or a token lock. A new call site fails
//! here until it is read the same way and added.

use std::fs;
use std::path::Path;

const MUTATORS: [&str; 5] = ["grant(", "revoke(", "install_spawn(", "join_group(", "rebind_address_space("];

/// (file, mutator, what the caller holds at the call)
const REVIEWED: &[(&str, &str, &str)] = &[
    ("src/syscall/microkernel/capability/handlers.rs", "grant(", "nothing: syscall context"),
    ("src/syscall/microkernel/capability/handlers.rs", "revoke(", "nothing: syscall context"),
    ("src/process/address_space/lifecycle/holder.rs", "rebind_address_space(", "RELEASE only, which is above MUTATE"),
    ("src/process/foreign/spawn_guest.rs", "install_spawn(", "nothing: the guest is built, not yet run"),
    ("src/process/core/table/create.rs", "rebind_address_space(", "nothing: before the table add"),
    ("src/process/core/table/thread_spawn.rs", "rebind_address_space(", "nothing: before the table add"),
    ("src/process/core/table/thread_spawn.rs", "join_group(", "nothing: after the table add, before it runs"),
    (
        "src/kernel_core/process_spawn/capsule_spawn/runner/install/install_caps.rs",
        "install_spawn(",
        "nothing: spawn install, before the run queue",
    ),
];

fn rust_files(dir: &Path, out: &mut Vec<std::path::PathBuf>) {
    for e in fs::read_dir(dir).expect("readable source tree").flatten() {
        let p = e.path();
        if p.is_dir() {
            rust_files(&p, out);
        } else if p.extension().is_some_and(|x| x == "rs") {
            out.push(p);
        }
    }
}

#[test]
fn every_call_into_the_caps_mutators_is_reviewed_for_lock_order() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut files = Vec::new();
    rust_files(&root.join("src"), &mut files);
    let mut found: Vec<(String, &str)> = Vec::new();
    for f in files {
        let rel = f.strip_prefix(&root).expect("under the root").to_string_lossy().replace('\\', "/");
        if rel.starts_with("src/process/caps/") {
            continue;
        }
        let text = fs::read_to_string(&f).expect("readable");
        for line in text.lines().filter(|l| !l.trim_start().starts_with("//")) {
            for m in MUTATORS {
                let named = [format!("caps::{m}"), format!("proc_caps::{m}")];
                if named.iter().any(|n| line.contains(n.as_str())) {
                    found.push((rel.clone(), m));
                }
            }
        }
    }
    found.sort();
    found.dedup();
    let mut want: Vec<(String, &str)> = REVIEWED.iter().map(|(f, m, _)| (f.to_string(), *m)).collect();
    want.sort();
    assert_eq!(found, want, "a call into process::caps's mutators not yet read for lock order");
}
