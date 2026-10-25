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

//! The write-back at drop as the writes it makes. Each write-back disables
//! and re-enables the adapter, which on QEMU clears video memory, so none
//! is made over a state the adapter still holds: the driver used to write
//! one back whatever the adapter showed. A state changed is written back
//! whole, in the order QEMU needs, which a memory window cannot witness.

use crate::constants::{DISPI_INDEX_BANK, DISPI_INDEX_BPP, DISPI_INDEX_ENABLE};
use crate::constants::{DISPI_INDEX_VIRT_WIDTH, DISPI_INDEX_XRES, DISPI_INDEX_X_OFFSET};
use crate::constants::{DISPI_INDEX_YRES, DISPI_INDEX_Y_OFFSET};
use crate::dispi::{write_back, Dispi};

/// A mode at XRES, YRES, BPP, ENABLE and BANK, its lines one mode wide.
const fn mode(xres: u16, yres: u16, bpp: u16, enable: u16, bank: u16) -> Dispi {
    Dispi { xres, yres, bpp, enable, bank, virt_width: xres, x_offset: 0, y_offset: 0 }
}

const OVMF: Dispi = mode(800, 600, 32, 0x41, 0);

fn writes(now: &Dispi, firmware: &Dispi) -> Vec<(u32, u16)> {
    let mut w = Vec::new();
    write_back(now, firmware, |index, value| w.push((index, value)));
    w
}

#[test]
fn nothing_is_written_back_over_a_state_the_adapter_still_holds() {
    let (text, set, zero) =
        (mode(640, 480, 8, 0, 2), mode(1024, 768, 32, 0x41, 0), mode(0, 0, 0, 0, 0));
    for held in [OVMF, text, set, zero] {
        assert_eq!(writes(&held, &held), [], "{held:?} written back over itself");
    }
}

#[test]
fn a_changed_state_is_written_back_whole_disabled_first_and_the_line_layout_last() {
    let panned = Dispi { bpp: 16, bank: 3, virt_width: 1280, x_offset: 8, y_offset: 2, ..OVMF };
    let (xres, yres, bpp) = (DISPI_INDEX_XRES, DISPI_INDEX_YRES, DISPI_INDEX_BPP);
    let (enable, bank, virt) = (DISPI_INDEX_ENABLE, DISPI_INDEX_BANK, DISPI_INDEX_VIRT_WIDTH);
    let order = [(enable, 0), (bank, 3), (xres, 800), (yres, 600), (bpp, 16), (enable, 0x41)];
    let layout = [(virt, 1280), (DISPI_INDEX_X_OFFSET, 8), (DISPI_INDEX_Y_OFFSET, 2)];
    let want: Vec<_> = order.into_iter().chain(layout).collect();
    assert_eq!(writes(&mode(1024, 768, 32, 0x41, 0), &panned), want);
    for changed in [Dispi { bank: 1, ..OVMF }, Dispi { y_offset: 1, ..OVMF }] {
        assert_eq!(writes(&changed, &OVMF).len(), 9, "{changed:?} not written back whole");
    }
}
