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

//! The futex glue pinned whole, comments and whitespace aside (L81): the
//! sweep teardown runs, the end a wake and a pass-on ask, and the pass-on's
//! own call, each as the kernel has it, so weakening any body is seen.

const QUEUE: &str = include_str!("../../../../src/syscall/microkernel/futex/queue.rs");
const WAIT: &str = include_str!("../../../../src/syscall/microkernel/futex/wait.rs");

/// The code alone: comments cut, whitespace dropped.
fn code(text: &str) -> String {
    let cut = |l: &str| l.split("//").next().unwrap_or("").split_whitespace().collect::<String>();
    text.lines().map(cut).collect()
}

/// The code of `text` from `from` up to `to`.
fn between(text: &str, from: &str, to: &str) -> String {
    let (c, from, to) = (code(text), code(from), code(to));
    let a = c.find(&from).unwrap_or_else(|| panic!("missing `{from}`"));
    let b = c[a..].find(&to).unwrap_or_else(|| panic!("missing `{to}`"));
    c[a..a + b].to_string()
}

#[test]
fn the_sweep_and_the_end_asked_are_the_kernels_own() {
    let sweep = concat!(
        "pub fn forget(pid: u32) { FUTEX_QUEUE.lock().retain(|_, waiting| {",
        " waiting.retain(|&p| p != pid); !waiting.is_empty() }); }",
    );
    assert_eq!(between(QUEUE, "pub fn forget(", "pub(super) fn ended("), code(sweep));
    let end = concat!(
        "pub(super) fn ended(pid: u32) -> bool { use crate::process::core::ProcessState;",
        " crate::process::nonos_core::PROCESS_TABLE.find_by_pid(pid).is_none_or(|pcb| {",
        " matches!(*pcb.state.lock(), ProcessState::Zombie(_) | ProcessState::Terminated(_))",
        " }) }",
    );
    let c = code(QUEUE);
    assert_eq!(&c[c.find(&code("pub(super) fn ended(")).expect("ended")..], code(end));
}

#[test]
fn a_waiter_returning_early_passes_its_wake_to_the_living() {
    let call = "let next = leave_early(&mut FUTEX_QUEUE.lock(), key, caller, super::queue::ended);";
    assert!(code(WAIT).contains(&code(call)), "the pass-on asks no end, or another one");
}
