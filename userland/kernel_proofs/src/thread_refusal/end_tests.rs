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

//! A thread published but never started is ended there, and one whose
//! process was killed while it was built is ended before it can run (L77).

const SPAWN: &str = include_str!("../../../../src/process/core/table/thread_spawn.rs");
const END: &str = include_str!("../../../../src/process/core/table/thread_end.rs");

fn at(text: &str, needle: &str) -> usize {
    text.find(needle).unwrap_or_else(|| panic!("missing `{needle}`"))
}

#[test]
fn a_thread_of_a_process_killed_while_it_was_built_never_runs() {
    let body = &SPAWN[at(SPAWN, "pub fn spawn_thread_parked")..];
    let published = &body[at(body, "PROCESS_TABLE.add(pcb);")..];
    let first = &published[..at(published, "register_inbox")];
    assert!(first.contains("if parent_ended(&parent) {"), "asked first after the add");
    assert!(first.contains("return Err(unstarted(tid, \"process ended\"));"));
    let ended = &END[at(END, "fn parent_ended(")..];
    assert!(ended.contains("ProcessState::Zombie(_) | ProcessState::Terminated(_)"));
    let undo = &END[at(END, "fn unstarted(")..];
    assert!(undo[..at(undo, "\n}")].contains("crate::process::exit::teardown(tid, "));
}
