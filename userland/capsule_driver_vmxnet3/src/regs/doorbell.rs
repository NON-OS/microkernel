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

//! The pass-through page: the interrupt mask and the producer doorbells.

use super::device::Regs;
use crate::constants::regs::{IMR, RXPROD, RXPROD2, TXPROD};

// SAFETY anchor: see device.rs.
impl Regs {
    /// Interrupt 0 masked: the driver polls, and intrCtrl disables all.
    pub fn mask(&self) {
        unsafe { self.pt.w32(IMR, 1) };
    }

    pub fn tx_prod(&self, next: u32) {
        unsafe { self.pt.w32(TXPROD, next) };
    }

    pub fn rx_prod(&self, ring0: u32, ring1: u32) {
        unsafe {
            self.pt.w32(RXPROD, ring0);
            self.pt.w32(RXPROD2, ring1);
        }
    }
}
