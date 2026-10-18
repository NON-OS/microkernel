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

//! The window the driver's `Regs` reaches, answered by this thread's part.
//! The base is ignored: each test installs one part on its own thread.

use crate::model::with_part;

#[derive(Clone, Copy)]
pub struct Mmio {
    _base: u64,
}

impl Mmio {
    pub const fn new(base: u64) -> Self {
        Self { _base: base }
    }

    /// # Safety
    /// None needed on the host; kept unsafe to match the shipping signature.
    pub unsafe fn r16(&self, offset: usize) -> u16 {
        with_part(|p| p.read16(offset))
    }

    /// # Safety
    /// As for `r16`.
    pub unsafe fn w16(&self, offset: usize, value: u16) {
        with_part(|p| p.write16(offset, value));
    }
}
