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

//! Whether a port reads stopped: PxCMD.ST, CR, FRE and FR all clear (AHCI
//! 1.3.1, 10.1.2), and no slot issued or queued in PxCI or PxSACT. After the
//! HBA reset that must hold; a port where it does not may fetch a command
//! list, or post a received FIS, at the addresses a dead driver gave it, or
//! run a stale slot once started. Such a port gets a stop of its own before
//! the controller is judged.

use crate::constants::regs::{CMD_CR, CMD_FR, CMD_FRE, CMD_ST, PORT_BASE, PORT_CMD, PORT_STRIDE};
use crate::constants::regs::{PORT_CI, PORT_SACT};
use crate::constants::MAX_PORTS;
use crate::engine::stop::stop as stop_port;
use crate::regs::Regs;

/// The PxCMD bits that show a port's engines on.
const ENGINE_BITS: u32 = CMD_ST | CMD_CR | CMD_FRE | CMD_FR;

/// The register block of each port in `pi`, of the first `count`.
fn ports(pi: u32, count: u8) -> impl Iterator<Item = (u8, u32)> {
    (0..core::cmp::min(count as usize, MAX_PORTS))
        .filter(move |&i| pi & (1u32 << i) != 0)
        .map(|i| (i as u8, PORT_BASE + i as u32 * PORT_STRIDE))
}

fn armed(regs: Regs, base: u32) -> bool {
    // SAFETY: `regs` maps this controller's ABAR, and `base` is a port's
    // register block inside it: `ports` yields only ports of the windowed PI.
    let r = |off| unsafe { regs.r32(base + off) };
    r(PORT_CMD) & ENGINE_BITS != 0 || r(PORT_CI) | r(PORT_SACT) != 0
}

/// The first port in `pi`, of the first `count`, whose PxCMD shows an
/// engine on.
pub fn armed_port(regs: Regs, pi: u32, count: u8) -> Option<u8> {
    ports(pi, count).find(|&(_, base)| armed(regs, base)).map(|(i, _)| i)
}

/// Stop every such port: ST off and CR seen clear, then FRE off and FR
/// seen clear, each within ENGINE_STOP_MS. Whether they cleared is for
/// `armed_port` to read again.
pub fn stop_armed(regs: Regs, pi: u32, count: u8) {
    for (_, base) in ports(pi, count).filter(|&(_, base)| armed(regs, base)) {
        let _ = stop_port(regs, base);
    }
}
