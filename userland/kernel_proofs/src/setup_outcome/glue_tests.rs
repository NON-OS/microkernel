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

//! The kernel calls the rules: teardown keeps the status as it records it,
//! setup's pid is asked for before setup can end, and init reads the kept
//! status.

const REAP_LOG: &str = include_str!("../../../../src/process/exit/reap_log.rs");
const STATE: &str = include_str!("../../../../src/userspace/capsule_setup_wizard/state.rs");
const OUTCOME: &str = include_str!("../../../../src/userspace/capsule_setup_wizard/outcome.rs");
const TEARDOWN: &str = include_str!("../../../../src/process/exit/teardown.rs");

fn body<'a>(src: &'a str, head: &str) -> &'a str {
    let at = src.find(head).unwrap_or_else(|| panic!("no {head}"));
    &src[at..at + src[at..].find("\n}\n").expect("no end")]
}

#[test]
fn teardown_keeps_the_status_as_it_records_it() {
    let record = body(REAP_LOG, "pub(super) fn record(");
    assert!(record.contains("super::kept::note(pid, code);"));
    let t = body(TEARDOWN, "pub fn teardown(");
    let zombie = t.find("ProcessState::Zombie(exit_code)").expect("zombie");
    let kept = t.find("super::reap_log::record(").expect("record");
    let gone = t.find("super::forget::forget(pid)").expect("forget");
    assert!(zombie < kept && kept < gone, "kept while the process is still a zombie");
}

#[test]
fn setups_pid_is_asked_for_when_it_is_spawned() {
    let alive = body(STATE, "pub(super) fn set_alive(");
    assert!(alive.contains("crate::process::exit::keep_status_of(pid);"));
}

#[test]
fn init_reads_the_kept_status() {
    let ended = body(OUTCOME, "pub fn ended(");
    assert!(ended.contains("read(pid, crate::process::exit::kept_status(pid), pcb, logged)"));
}
