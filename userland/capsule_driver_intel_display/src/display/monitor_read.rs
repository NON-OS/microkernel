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

//! The EDID of the monitor on a lit output, read the way its port speaks:
//! I2C over the DisplayPort AUX channel (VESA DisplayPort 1.4, section
//! 2.7.7) for DisplayPort and eDP, DDC through GMBUS for HDMI and DVI.
//! Blocks are taken whole or not at all (`monitor_block`): the base, then
//! as many extensions as it, or its HF-EEODB, declares, up to BLOCKS. A
//! block the monitor cannot give ends the read with the blocks before it,
//! and one that does not answer gives an empty blob, never an error.

use super::monitor_block::block;
use crate::engine::access::{Clock, Mmio};
use crate::engine::gmbus::BLOCK;
use crate::engine::gmbus_xfer::deadline::Deadline;
use crate::engine::route::Route;
use nonos_edid::cta::eeodb;

/// The most blocks read: segments 0 and 1, room for the base and the
/// three extensions an HDMI 2.1 sink's HF-EEODB most often names.
pub const BLOCKS: usize = 4;
pub const EDID_MAX: usize = BLOCKS * BLOCK;
/// The whole read, every block and every try. An honest sink gives four
/// blocks in about 50 ms; a stalling one is cut off here.
pub const EDID_US: u64 = 400_000;

/// The bytes read into `out`: a whole number of blocks, at most EDID_MAX.
/// Nothing past them is written.
pub fn read(m: &mut dyn Mmio, clock: &dyn Clock, r: &Route, out: &mut [u8; EDID_MAX]) -> usize {
    let whole = Deadline::new(clock, EDID_US);
    let mut n = 0;
    while n < BLOCKS && n <= declared(&out[..n * BLOCK]) {
        let (before, rest) = out.split_at_mut(n * BLOCK);
        let Some(b) = block(m, clock, r, n as u8, before, &whole) else { break };
        rest[..BLOCK].copy_from_slice(&b);
        n += 1;
    }
    n * BLOCK
}

/// The extensions the blocks in hand declare: none before the base, its
/// byte 126 alone, then block 1's HF-EEODB count when it carries one.
fn declared(have: &[u8]) -> usize {
    match have.len() / BLOCK {
        0 => 0,
        1 => have[126] as usize,
        _ => eeodb::count(have).unwrap_or(have[126] as usize),
    }
}
