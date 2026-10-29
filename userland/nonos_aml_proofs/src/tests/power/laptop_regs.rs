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

//! The laptop's embedded controller: where its registers sit, and the
//! tables loaded with them holding a reading.

use super::laptop::tables;
use crate::rig::Rig;

/// The EC's register map: state, the AC (bit 0) and lid (bit 1) byte, then
/// 16-bit words: rate, remaining, last full, design, voltage.
pub const STATE: u64 = 0x40;
pub const BITS: u64 = 0x41;
pub const RATE: u64 = 0x42;
pub const LEFT: u64 = 0x44;
pub const FULL: u64 = 0x46;
pub const DESIGN: u64 = 0x48;
pub const VOLTS: u64 = 0x4A;

/// The tables loaded with every space connected, and the EC's bytes: each
/// register's value, a 16-bit word from RATE on.
pub fn laptop(regs: &[(u64, u16)]) -> Rig {
    let mut r = Rig::new(&tables()).connected();
    for &(at, v) in regs {
        r.probe.bytes.insert((3, at), v as u8);
        if at >= RATE {
            r.probe.bytes.insert((3, at + 1), (v >> 8) as u8);
        }
    }
    r
}
