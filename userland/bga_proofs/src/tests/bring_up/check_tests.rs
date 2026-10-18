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

//! What the adapter must say back, at and past every bound: the DISPI ID
//! register before a mode is set, and the mode registers after.
//!
//! The mode used to be written blind: anything behind the Bochs PCI ID was
//! programmed as DISPI, and a mode the adapter clamped or dropped kept as
//! taken. The read-back is held against windows holding another mode.

use nonos_devmodel::FakeBar;

use crate::constants::{
    DISPI_BPP_32, DISPI_ENABLED, DISPI_INDEX_BPP, DISPI_INDEX_ENABLE, DISPI_INDEX_ID,
    DISPI_INDEX_VIRT_WIDTH, DISPI_INDEX_XRES, DISPI_INDEX_YRES, DISPI_LFB_ENABLED,
};
use crate::dispi::{current, dispi_off, identify, DispiMode};
use crate::error::BgaError;
use crate::regs::Regs;

/// A DISPI block holding `id` and the mode `(xres, yres, bpp, enable)`,
/// one mode-wide line after another.
fn window(id: u16, mode: (u16, u16, u16, u16)) -> FakeBar {
    let bar = FakeBar::new(0x600);
    let at = |index| dispi_off(index) as usize;
    bar.present16(at(DISPI_INDEX_ID), id);
    bar.present16(at(DISPI_INDEX_VIRT_WIDTH), mode.0);
    bar.present16(at(DISPI_INDEX_XRES), mode.0);
    bar.present16(at(DISPI_INDEX_YRES), mode.1);
    bar.present16(at(DISPI_INDEX_BPP), mode.2);
    bar.present16(at(DISPI_INDEX_ENABLE), mode.3);
    bar
}

const ON: u16 = DISPI_ENABLED | DISPI_LFB_ENABLED;
const SET: (u16, u16, u16, u16) = (1024, 768, DISPI_BPP_32, ON);

#[test]
fn only_the_dispi_versions_0xb0c0_to_0xb0c5_are_driven() {
    for id in [0xB0C0, 0xB0C3, 0xB0C5] {
        assert_eq!(identify(Regs::new(window(id, SET).base())), Ok(()), "{id:#x} refused");
    }
    for id in [0, 0xB0BF, 0xB0C6, 0xB0CF, 0x0C05, 0xFFFF] {
        let r = identify(Regs::new(window(id, SET).base()));
        assert_eq!(r, Err(BgaError::NotDispi), "{id:#x} taken for a DISPI adapter");
    }
}

#[test]
fn a_mode_reads_back_as_set_only_when_the_adapter_holds_it() {
    let want = Some(DispiMode { width: 1024, height: 768, stride: 4096 });
    let held = |mode| current(Regs::new(window(0xB0C5, mode).base())) == want;
    assert!(held(SET));
    assert!(held((1024, 768, 32, 0xffff)), "ENABLE's other bits are the adapter's");
    let clamped = [(800, 768, 32, ON), (1024, 600, 32, ON), (1024, 768, 16, ON)];
    let off =
        [(1024, 768, 32, DISPI_ENABLED), (1024, 768, 32, DISPI_LFB_ENABLED), (1024, 768, 32, 0)];
    for mode in clamped.into_iter().chain(off) {
        assert!(!held(mode), "{mode:?} taken for 1024x768x32");
    }
}
