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

//! The CSRs by number, and the two poll demands: a write of anything to
//! CSR1 or CSR2 sends the transmit or receive process to look at its ring
//! now, and wakes it if it had suspended for want of a descriptor.

use super::mmio::Mmio;
use crate::constants::csr::{CSR1, CSR2};

#[derive(Clone, Copy)]
pub struct Regs {
    io: Mmio,
}

// SAFETY anchor for every unsafe below: `io` maps the whole 128-byte window,
// and CSR0 to CSR15 are longwords inside it.
impl Regs {
    pub const fn new(base: u64) -> Self {
        Self { io: Mmio::new(base) }
    }

    pub fn csr(&self, n: u16) -> u32 {
        unsafe { self.io.r32(n as usize * 8) }
    }

    pub fn set_csr(&self, n: u16, value: u32) {
        unsafe { self.io.w32(n as usize * 8, value) }
    }

    pub fn tx_poll(&self) {
        self.set_csr(CSR1, 1);
    }

    pub fn rx_poll(&self) {
        self.set_csr(CSR2, 1);
    }
}
