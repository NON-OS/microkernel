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

use crate::proofs::proofs_8::transport_takeover::text::{between, code};

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
    // The whole pass-on, its wake included: waking anyone but `next` loses
    // the wake the leaver was handed (L81c).
    let pass = concat!(
        "let next = leave_early(&mut FUTEX_QUEUE.lock(), key, caller, super::queue::ended);",
        " if let Some(next) = next { crate::sched::wake_process(next); }",
    );
    let got = between(WAIT, "let next = leave_early(", "return if read.is_err()");
    assert_eq!(got, code(pass), "the pass-on asks no end, or wakes another");
}
