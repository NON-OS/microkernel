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

//! The child's half of the separation probe.

use crate::sep_probe::{peek, poke};
use crate::sys::{call, NANOSLEEP, PROCESS_VM_READV, PTRACE};

/// Attaches without stopping the target, so a success does not wedge it.
const PTRACE_SEIZE: u64 = 0x4206;

/// Bits of the exit status: 1 saw the parent's later write, 2 reached into
/// the parent through a call.
pub fn run(parent: u32) -> u64 {
    // Long enough for the parent's write to land, if it were going to.
    let tenth = [0u64, 200_000_000];
    let _ = call(NANOSLEEP, [tenth.as_ptr() as u64, 0, 0, 0, 0, 0]);
    let mut bits = 0u64;
    if peek() == b'B' {
        bits |= 1;
    }
    let mut buf = [0u8; 1];
    let local = [buf.as_mut_ptr() as u64, 1u64];
    let remote = [0x5000_0000u64, 1u64];
    let read = call(
        PROCESS_VM_READV,
        [parent as u64, local.as_ptr() as u64, 1, remote.as_ptr() as u64, 1, 0],
    );
    let traced = call(PTRACE, [PTRACE_SEIZE, parent as u64, 0, 0, 0, 0]);
    if read >= 0 || traced >= 0 {
        bits |= 2;
    }
    poke(b'C');
    bits
}
