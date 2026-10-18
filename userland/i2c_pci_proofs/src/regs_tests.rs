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

//! The register accessor stays inside the window it was given. An offset
//! past the end, one whose dword would straddle the end, a misaligned one or
//! one that overflows reads as an undecoded bus and writes nowhere.

use crate::model::WINDOW;
use crate::setup::regs;
use nonos_devmodel::FakeBar;

const LAST: u64 = WINDOW as u64 - 4;

#[test]
fn the_last_dword_of_the_window_is_reached() {
    let bar = FakeBar::new(WINDOW);
    let r = regs(&bar);
    r.write32(LAST, 0x1234_5678);
    assert_eq!(bar.wrote32(LAST as usize), 0x1234_5678);
    assert_eq!(r.read32(LAST), 0x1234_5678);
}

#[test]
fn offsets_outside_the_window_read_all_ones_and_write_nothing() {
    let bar = FakeBar::new(WINDOW);
    let r = regs(&bar);
    for offset in [WINDOW as u64, LAST + 2, LAST + 1, 2, u64::MAX - 3, u64::MAX] {
        assert_eq!(r.read32(offset), u32::MAX, "offset {offset:#x}");
        r.write32(offset, 0xA5A5_A5A5);
    }
    assert!((0..WINDOW).all(|o| bar.wrote8(o) == 0), "a refused write landed in the window");
}

#[test]
fn an_empty_window_refuses_every_access() {
    // SAFETY: an empty window: every access is refused before memory.
    let r = unsafe { crate::regs::Regs::new(0, 0) };
    assert_eq!(r.read32(0), u32::MAX);
    r.write32(0, 1);
}
