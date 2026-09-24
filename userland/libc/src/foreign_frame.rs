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

//! The register frame of one refused syscall. Mirrors the kernel's
//! `ForeignFrame` field for field; appended to, never reordered.
//!
//! The argument order is what a `syscall` instruction leaves behind under
//! the System V convention, so a supervisor reads its guest's arguments
//! without knowing anything about this kernel.

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ForeignFrame {
    pub pid: u32,
    pub _pad: u32,
    /// The number the guest asked for, verbatim and uninterpreted.
    pub nr: u64,
    pub arg0: u64,
    pub arg1: u64,
    pub arg2: u64,
    pub arg3: u64,
    pub arg4: u64,
    pub arg5: u64,
    /// Where the guest was, for diagnostics.
    pub rip: u64,
}

impl ForeignFrame {
    /// The six arguments in call order, for a handler that takes a slice.
    pub fn args(&self) -> [u64; 6] {
        [self.arg0, self.arg1, self.arg2, self.arg3, self.arg4, self.arg5]
    }
}
