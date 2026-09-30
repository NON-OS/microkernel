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

//! Letting the links come up after the HBA reset, before the ports are read.
//!
//! The reset restarts link initialization on the implemented ports, and a
//! real drive can take a moment to finish the out-of-band exchange. A port
//! read in that window shows DET = 1 (device seen, no communication yet) and
//! would be passed over as empty.

use nonos_libc::Deadline;

use crate::constants::regs::{PORT_BASE, PORT_SSTS, PORT_STRIDE};
use crate::constants::MAX_PORTS;
use crate::regs::Regs;

/// Long enough for the controller's own COMRESET to start on every port.
const FLOOR_MS: u64 = 10;
/// A link that has not settled by then is left for the walk to reset itself.
const LIMIT_MS: u64 = 1_000;

/// Wait until no implemented port in `pi` is between seeing a device and
/// talking to it, for at least FLOOR_MS and at most LIMIT_MS.
pub fn settle_links(regs: Regs, pi: u32, max_ports: u8) {
    let floor = Deadline::after_ms(FLOOR_MS);
    while !floor.expired() {
        core::hint::spin_loop();
    }
    let limit = Deadline::after_ms(LIMIT_MS);
    while any_detecting(regs, pi, max_ports) && !limit.expired() {
        core::hint::spin_loop();
    }
}

fn any_detecting(regs: Regs, pi: u32, max_ports: u8) -> bool {
    let count = core::cmp::min(max_ports as usize, MAX_PORTS);
    (0..count).filter(|i| pi & (1u32 << i) != 0).any(|i| {
        /*
         * SAFETY: eK@nonos.systems - `regs` maps this controller's ABAR and
         * PxSSTS of an implemented port lies inside it.
         */
        let ssts = unsafe { regs.r32(PORT_BASE + i as u32 * PORT_STRIDE + PORT_SSTS) };
        ssts & 0x0f == 1
    })
}
