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

//! The machine, on the host. The capsule's machine.rs asks the NONOS kernel
//! (MkProcStat) and stays target-only; prove.rs reads only the free memory
//! and the least it fell to while a proof ran, which the host reports as 0.

/// The machine's free memory in MiB: 0, the kernel not being there to ask.
pub fn mem_free_mib() -> u64 {
    0
}

/// A watch of the free memory with no kernel to read it from.
pub struct Watch;

impl Watch {
    pub fn start() -> Watch {
        Watch
    }

    /// The least free memory seen, in MiB: 0, as `mem_free_mib`.
    pub fn stop(self) -> u64 {
        0
    }
}
