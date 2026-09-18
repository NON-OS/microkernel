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

//! The register frame of one refused syscall, as handed to the supervisor.
//!
//! The field order is the System V argument order a `syscall` instruction
//! leaves behind, so a supervisor reads its guest's arguments without
//! knowing anything about this kernel. `pid` and `rip` are here so a
//! supervisor with several guests can tell them apart and can report where
//! an unserviceable call came from.

/// Wire layout shared with userspace. Appended to, never reordered.
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ForeignFrame {
    pub pid: u32,
    pub _pad: u32,
    /// The syscall number the guest asked for, verbatim.
    pub nr: u64,
    pub arg0: u64,
    pub arg1: u64,
    pub arg2: u64,
    pub arg3: u64,
    pub arg4: u64,
    pub arg5: u64,
    /// The instruction after the guest's `syscall`, for diagnostics.
    pub rip: u64,
}

impl ForeignFrame {
    pub(super) fn new(pid: u32, nr: u64, args: [u64; 6], rip: u64) -> Self {
        ForeignFrame {
            pid,
            _pad: 0,
            nr,
            arg0: args[0],
            arg1: args[1],
            arg2: args[2],
            arg3: args[3],
            arg4: args[4],
            arg5: args[5],
            rip,
        }
    }
}
