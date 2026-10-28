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

//! A process's arguments are set inside its spawn, before it is queued to
//! run, never after the spawn returns: set after, they raced its first
//! instruction, and on KVM an install read none and ran in a program's heap.
//! So `set_argv` is called by the spawn's admit step and the terminal run's
//! admission alone, and the admit step comes before the run queue.

use std::fs;
use std::path::{Path, PathBuf};

const INSTALL: &str =
    include_str!("../../../src/kernel_core/process_spawn/capsule_spawn/runner/install/install.rs");
const ADMIT: &str =
    include_str!("../../../src/kernel_core/process_spawn/capsule_spawn/runner/install/admit.rs");

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

fn sets_argv(text: &str) -> bool {
    let call = |l: &&str| l.contains("set_argv(") && !l.contains("fn set_argv(");
    text.lines().filter(|l| !l.trim_start().starts_with("//")).any(|l| call(&l))
}

#[test]
fn only_the_spawn_sets_a_process_arguments() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../src");
    let mut files = Vec::new();
    sources(&root, &mut files);
    let mut callers: Vec<String> = files
        .iter()
        .filter(|p| sets_argv(&fs::read_to_string(p).expect("a source file")))
        .map(|p| p.strip_prefix(&root).expect("under src").display().to_string())
        .collect();
    callers.sort();
    let admit = "kernel_core/process_spawn/capsule_spawn/runner/install/admit.rs";
    assert_eq!(callers, [admit, "userspace/capsule_linux/terminal/admit.rs"]);
}

#[test]
fn the_arguments_are_set_before_the_process_is_queued() {
    let admit = INSTALL.find("super::admit::admit(params, pid);").expect("the admit step");
    let queued = INSTALL.find("crate::sched::add_to_run_queue(pid);").expect("the run queue");
    assert!(admit < queued, "the process is queued before it has its arguments");
    assert!(!INSTALL[admit..queued].contains('?'), "a step between them can fail");
    assert!(ADMIT.contains("crate::process::set_argv(pid, argv.clone());"));
}
