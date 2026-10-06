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

//! The FPU state of a guest thread inside a signal handler, kept in the
//! kernel: entering saves the thread's own and gives the handler a clean
//! unit, returning puts it back, and no guest can forge it.

use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use spin::Mutex;

use crate::process::signal::SIGSEGV;
use crate::process::userspace::types::FpuState;

/// Handlers nested deeper than this end the thread.
const DEPTH: usize = 8;

static SAVED: Mutex<BTreeMap<u32, Vec<Box<FpuState>>>> = Mutex::new(BTreeMap::new());

/// Keep the thread's FPU state for its handler's return and give the handler
/// a clean unit. Handlers nested past `DEPTH` end the thread.
pub(super) fn enter_fpu(pid: u32) {
    let mut fpu = FpuState::new();
    fpu.save();
    let pushed = {
        let mut saved = SAVED.lock();
        let stack = saved.entry(pid).or_default();
        let room = stack.len() < DEPTH;
        if room {
            stack.push(fpu);
        }
        room
    };
    if !pushed {
        crate::process::terminate_current_with_signal(SIGSEGV);
    }
    FpuState::init();
}

/// Put back the state the innermost handler was entered over. A return with
/// nothing delivered is not a frame this kernel made, and ends the thread.
pub(super) fn leave_fpu(pid: u32) {
    let top = SAVED.lock().get_mut(&pid).and_then(|s| s.pop());
    match top {
        Some(fpu) => fpu.restore(),
        None => crate::process::terminate_current_with_signal(SIGSEGV),
    }
}

/// Exec and exit leave no handler to return from.
pub(super) fn forget(pid: u32) {
    SAVED.lock().remove(&pid);
}
