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

//! DISPI as firmware leaves it and as an attempt leaves it, on the adapter
//! and in the shim's record of the DISPI block at each give-back.

use nonos_libc::{given_back, registers_at_give_back};

use super::adapter::Adapter;
use crate::constants::{DISPI_BPP_32, DISPI_ENABLED, DISPI_LFB_ENABLED};
use crate::constants::{DISPI_INDEX_BPP, DISPI_INDEX_ENABLE, DISPI_INDEX_VIRT_WIDTH};
use crate::constants::{DISPI_INDEX_ID, DISPI_INDEX_XRES, DISPI_INDEX_YRES};
use crate::dispi::dispi_off;

/// The DISPI indices a driver may write, by number as the interface
/// publishes them: XRES, YRES, BPP, ENABLE, BANK, VIRT_WIDTH, X_OFFSET and
/// Y_OFFSET. VIRT_HEIGHT (7) is the adapter's own sum and takes no write.
const STATE: [u32; 8] = [1, 2, 3, 4, 5, 6, 8, 9];

/// DISPI as OVMF leaves it for a `w` x `h` GOP framebuffer.
pub fn showing(a: &Adapter, w: u16, h: u16) {
    let at = |index| dispi_off(index) as usize;
    a.regs.present16(at(DISPI_INDEX_XRES), w);
    a.regs.present16(at(DISPI_INDEX_YRES), h);
    a.regs.present16(at(DISPI_INDEX_BPP), DISPI_BPP_32);
    a.regs.present16(at(DISPI_INDEX_ENABLE), DISPI_ENABLED | DISPI_LFB_ENABLED);
    a.regs.present16(at(DISPI_INDEX_VIRT_WIDTH), w);
}

/// DISPI as a BIOS leaves it under VGA text: off, with stale values.
pub fn text_mode(a: &Adapter) {
    for (index, value) in STATE.into_iter().zip([640, 480, 8, 0, 2, 640, 0, 0]) {
        a.regs.present16(dispi_off(index) as usize, value);
    }
}

/// Every DISPI register a driver may write, as the adapter holds it.
pub fn state(a: &Adapter) -> [u16; 8] {
    STATE.map(|index| a.regs.wrote16(dispi_off(index) as usize))
}

/// That the attempt given back after the `before`th give-back wrote
/// `firmware` back with the registers still mapped: from its first unmap on,
/// every give-back found the adapter holding it, and it holds it still.
pub fn written_back(a: &Adapter, before: usize, firmware: [u16; 8]) {
    let back = given_back();
    let first = (before..back.len()).find(|&i| back[i].0 == "mmio_unmap");
    let first = first.expect("no mapping was given back");
    for (i, seen) in registers_at_give_back().iter().enumerate().skip(first) {
        assert_eq!(recorded(seen), firmware, "not written back by {} ({i})", back[i].0);
    }
    assert_eq!(state(a), firmware, "the adapter was left in another state");
}

/// The eight registers in a record of the DISPI block, which starts at ID.
fn recorded(block: &[u8]) -> [u16; 8] {
    let start = dispi_off(DISPI_INDEX_ID);
    STATE.map(|index| {
        let at = (dispi_off(index) - start) as usize;
        block.get(at..at + 2).map_or(0, |b| u16::from_le_bytes([b[0], b[1]]))
    })
}
