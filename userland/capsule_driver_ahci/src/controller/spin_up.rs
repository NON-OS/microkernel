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

//! Starting every port's link after the HBA reset.
//!
//! On an HBA with staggered spin-up (CAP.SSS, which most laptop PCHs set,
//! Gemini Lake's among them), the reset leaves PxCMD.SUD clear on every port.
//! With SUD clear the port sends no COMRESET, so its link never starts and
//! PxSSTS.DET reads 0 whether a disk is there or not. A walk that brought up
//! only ports whose DET already read 3 found no disk at all. Linux sets SUD
//! (and POD where cold presence detection makes it writable) on every
//! implemented port before it resets the link (ahci_power_up); so does this.

use super::ports::spin_up_bits;
use crate::constants::regs::{PORT_BASE, PORT_CMD, PORT_STRIDE};
use crate::constants::MAX_PORTS;
use crate::regs::Regs;

/// Set PxCMD.SUD, and PxCMD.POD when CAP.CPD, on every port in `pi`.
pub fn spin_up(regs: Regs, cap: u32, pi: u32, count: u8) {
    let bits = spin_up_bits(cap);
    for i in (0..core::cmp::min(count as usize, MAX_PORTS)).filter(|i| pi & (1u32 << i) != 0) {
        let at = PORT_BASE + i as u32 * PORT_STRIDE + PORT_CMD;
        /*
         * SAFETY: eK@nonos.systems - `regs` maps this controller's ABAR and
         * `pi` holds only ports whose registers lie inside the window.
         */
        unsafe {
            regs.w32(at, regs.r32(at) | bits);
        }
    }
}
