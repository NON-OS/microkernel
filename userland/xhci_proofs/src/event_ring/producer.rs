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

//! The controller's side of the event ring (xHCI 1.2 section 4.9.4).
//!
//! It finds the ring the way hardware does, through the segment table the
//! driver programmed into ERSTBA and ERSTSZ, and never through the driver's
//! own fields. It writes each event with its producer cycle state, flips that
//! state when it wraps at the segment size the table gives, and refuses to
//! write over an event the driver has not handed back through ERDP: the ring
//! is full when the enqueue pointer is one behind the dequeue pointer.
//! Finding the ring is `producer_start`, where the driver stands
//! `producer_ring`, and writing events `producer_write`.

use nonos_devmodel::FakeBar;

use crate::constants::TRB_BYTES;

pub use super::producer_write::is_poison;

/// IMAN through the top of ERDP.
pub const INTERRUPTER_BYTES: usize = 0x20;
/// The low four bits of ERDP: DESI in 2:0 and EHB in 3.
pub const ERDP_FLAGS: u64 = 0xF;
pub const ERDP_EHB_BIT: u64 = 1 << 3;
pub(super) const TRB: u64 = TRB_BYTES as u64;

pub fn reg64(bar: &FakeBar, off: u64) -> u64 {
    let off = off as usize;
    bar.wrote32(off) as u64 | (bar.wrote32(off + 4) as u64) << 32
}

pub struct Producer {
    pub regs: FakeBar,
    pub(super) segment_bus: u64,
    pub(super) segment_host: u64,
    pub(super) trbs: usize,
    pub(super) enqueue: usize,
    pub(super) cycle: bool,
}

impl Producer {
    /// The interrupter's registers, as yet unprogrammed.
    pub fn new() -> Self {
        Self {
            regs: FakeBar::new(INTERRUPTER_BYTES),
            segment_bus: 0,
            segment_host: 0,
            trbs: 0,
            enqueue: 0,
            cycle: true,
        }
    }
}
