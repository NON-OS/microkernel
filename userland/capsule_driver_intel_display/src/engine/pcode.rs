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

//! The memory latencies the watermarks are built from, asked of the power
//! controller through its mailbox (SKL PRM Vol 2c, GEN6_PCODE_MAILBOX and
//! GEN6_PCODE_DATA; Vol 12, watermark latency read, command 0x6): data 0
//! selects levels 0-3, data 1 levels 4-7, and each answer holds four
//! latencies of a byte each, in microseconds. Command and register layout
//! unverified against the volumes.

use super::access::{Expiry, Mmio};
use super::error::Error;
use super::wait::wait_for;

pub const MAILBOX: u32 = 0x13_8124;
pub const DATA: u32 = 0x13_8128;
pub const DATA1: u32 = 0x13_812C;
pub const READY: u32 = 1 << 31;
pub const READ_MEM_LATENCY: u32 = 0x6;

fn ask(m: &mut dyn Mmio, cmd: u32, data: u32, until: &dyn Expiry) -> Result<u32, Error> {
    wait_for(m, MAILBOX, READY, 0, until)?;
    m.write(DATA, data);
    m.write(DATA1, 0);
    m.write(MAILBOX, READY | cmd);
    wait_for(m, MAILBOX, READY, 0, until)?;
    Ok(m.read(DATA))
}

/// The eight latencies, level 0 first. A level reporting 0 has none and
/// disables itself and every level above it.
pub fn latencies(m: &mut dyn Mmio, until: &dyn Expiry) -> Result<[u32; 8], Error> {
    let lo = ask(m, READ_MEM_LATENCY, 0, until)?.to_le_bytes();
    let hi = ask(m, READ_MEM_LATENCY, 1, until)?.to_le_bytes();
    let mut l = [0u32; 8];
    for i in 0..4 {
        l[i] = lo[i] as u32;
        l[4 + i] = hi[i] as u32;
    }
    if let Some(z) = l.iter().position(|&v| v == 0) {
        l[z..].fill(0);
    }
    Ok(l)
}
