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

//! The selftest capsule's entry: heap, waiting for vfs, the run, the exit.

use nonos_libc::{heap_init, mk_debug, mk_exit, mk_yield, HeapError};

use super::{run, run_cmd};
use crate::term::state::State;

const READY_ATTEMPTS: u32 = 100_000;

// Headless capsule entry for the autorun-selftest build: bring up the heap,
// wait until vfs answers, drive the unproven shell paths, then exit. No
// surface, no compositor, no focus handoff -- this grades shell logic, not
// the GUI app loop.
pub fn main() -> ! {
    match heap_init() {
        Ok(()) | Err(HeapError::AlreadyInitialized) => {}
        Err(_) => exit_fail(b"[TERMINAL-TEST] FAIL heap\n"),
    }
    let mut state = State::new();
    let mut attempts = 0u32;
    while !ready(&mut state) {
        attempts += 1;
        if attempts >= READY_ATTEMPTS {
            exit_fail(b"[TERMINAL-TEST] FAIL vfs never ready\n");
        }
        mk_yield();
    }
    run(&mut state);
    mk_exit(0);
}

fn exit_fail(msg: &[u8]) -> ! {
    let _ = mk_debug(msg.as_ptr(), msg.len());
    mk_exit(1);
}

// True once `read` of a vfs-seeded file succeeds, so the assertions only
// run after the capsule stack has settled and vfs is answering.
fn ready(state: &mut State) -> bool {
    run_cmd(state, b"read /readme.txt");
    state.last_status == 0
}
