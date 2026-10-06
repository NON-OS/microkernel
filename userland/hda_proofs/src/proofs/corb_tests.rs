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
//! Describing the command and response rings to the controller.
//!
//! Both rings are named by a 64-bit physical address split across two
//! registers, and the controller fetches from them by DMA. An address landing
//! in the wrong half aims a bus master at memory the driver did not allocate.

use nonos_devmodel::FakeBar;

use crate::constants::{
    CORBLBASE, CORBSIZE, CORBUBASE, RINGSIZE_16, RINGSIZE_256,
    RINGSIZE_CAP_16, RINGSIZE_CAP_2, RINGSIZE_CAP_256, RIRBLBASE, RIRBSIZE, RIRBUBASE,
};
use crate::controller::corb::{self, ring_size, Rings};
use crate::model::WINDOW;
use crate::regs::Regs;

/// Two 128-byte-aligned ring addresses, deliberately unlike each other in both
/// halves so a swap or a truncation cannot read as a match.
pub const CORB_PA: u64 = 0x0000_0001_2345_6000;
pub const RIRB_PA: u64 = 0x0000_0007_ABCD_E080;

/// A window a full ring bring-up has already run against.
pub fn brought_up() -> FakeBar {
    let bar = FakeBar::new(WINDOW);
    let rings = corb::init(Regs::new(bar.base()), CORB_PA, RIRB_PA);
    assert_eq!(rings, Rings { entries: 256, rp_handshake: true }, "ring bring-up");
    bar
}

#[test]
fn each_ring_address_reaches_the_controller_whole_and_in_the_right_halves() {
    let bar = brought_up();
    assert_eq!(bar.wrote32(CORBLBASE as usize), CORB_PA as u32);
    assert_eq!(bar.wrote32(CORBUBASE as usize), (CORB_PA >> 32) as u32);
    assert_eq!(bar.wrote32(RIRBLBASE as usize), RIRB_PA as u32);
    assert_eq!(bar.wrote32(RIRBUBASE as usize), (RIRB_PA >> 32) as u32);
}

#[test]
fn the_ring_size_is_the_largest_the_controller_offers() {
    assert_eq!(ring_size(RINGSIZE_CAP_256 | RINGSIZE_CAP_16 | RINGSIZE_CAP_2), (RINGSIZE_256, 256));
    assert_eq!(ring_size(RINGSIZE_CAP_16 | RINGSIZE_CAP_2), (RINGSIZE_16, 16));
    assert_eq!(ring_size(RINGSIZE_CAP_2), (0, 2));
    // No capability bits at all: 256, the size every controller Linux drives takes.
    assert_eq!(ring_size(0), (RINGSIZE_256, 256));
}

#[test]
fn a_controller_offering_only_sixteen_entries_gets_sixteen_on_both_rings() {
    let bar = FakeBar::new(WINDOW);
    bar.present8(CORBSIZE as usize, RINGSIZE_CAP_16);
    bar.present8(RIRBSIZE as usize, RINGSIZE_CAP_16);
    let rings = corb::init(Regs::new(bar.base()), CORB_PA, RIRB_PA);
    assert_eq!(rings.entries, 16, "the driver would count to 256 on a 16-entry ring");
    assert_eq!(bar.wrote8(CORBSIZE as usize) & 3, RINGSIZE_16);
    assert_eq!(bar.wrote8(RIRBSIZE as usize) & 3, RINGSIZE_16);
}
