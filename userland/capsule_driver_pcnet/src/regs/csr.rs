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

//! The indexed registers. RAP selects a CSR or BCR and RDP or BDP carries
//! its value, so each access is two bus cycles. The capsule is one thread,
//! which is what keeps a RAP write and its data cycle together.

use super::mmio::Mmio;
use crate::constants::regs::{BDP, RAP, RDP, RESET};

#[derive(Clone, Copy)]
pub struct Regs {
    io: Mmio,
}

// SAFETY anchor for every unsafe below: `io` maps the whole 32-byte WIO
// window, and RAP, RDP, BDP and RESET are word registers inside it.
impl Regs {
    pub const fn new(base: u64) -> Self {
        Self { io: Mmio::new(base) }
    }

    pub fn csr(&self, n: u16) -> u16 {
        unsafe {
            self.io.w16(RAP, n);
            self.io.r16(RDP)
        }
    }

    pub fn set_csr(&self, n: u16, value: u16) {
        unsafe {
            self.io.w16(RAP, n);
            self.io.w16(RDP, value);
        }
    }

    pub fn bcr(&self, n: u16) -> u16 {
        unsafe {
            self.io.w16(RAP, n);
            self.io.r16(BDP)
        }
    }

    pub fn set_bcr(&self, n: u16, value: u16) {
        unsafe {
            self.io.w16(RAP, n);
            self.io.w16(BDP, value);
        }
    }

    /// A read of RESET is the software reset: the part stops, forgets its
    /// rings, and drops back to word I/O.
    pub fn reset(&self) {
        unsafe {
            let _ = self.io.r16(RESET);
        }
    }
}
