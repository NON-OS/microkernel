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

//! The events counted per process. The discriminant is the slot index.

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum Kind {
    /// One syscall entered, whatever it returned.
    Syscall = 0,
    /// One message handed to the kernel for another process.
    IpcTx = 1,
    /// One message copied out to this process.
    IpcRx = 2,
    /// One page fault taken while this process was current.
    Fault = 3,
    /// One switch onto the processor.
    Switch = 4,
    /// One timer tick that interrupted this process's own code.
    UserTick = 5,
}

impl Kind {
    pub const COUNT: usize = 6;
}
