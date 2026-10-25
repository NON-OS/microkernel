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

//! The modifier state of a key event, as the xkb masks a client is sent.

use crate::xkb_mods::{xkb_masks, ALT, ALTGR, CAPS, CTRL, META, NUM, SHIFT};

#[test]
fn each_modifier_is_its_core_xkb_bit() {
    let held = [(SHIFT, 1), (CTRL, 4), (ALT, 8), (META, 64), (ALTGR, 128)];
    held.iter().for_each(|(flag, mask)| assert_eq!(xkb_masks(*flag), (*mask, 0), "{flag:#x}"));
    assert_eq!(xkb_masks(CAPS), (0, 2));
    assert_eq!(xkb_masks(NUM), (0, 16));
    assert_eq!(xkb_masks(0), (0, 0));
}

#[test]
fn modifiers_combine_and_unknown_bits_add_nothing() {
    assert_eq!(xkb_masks(SHIFT | CTRL | CAPS), (5, 2));
    assert_eq!(xkb_masks(0xFF80), (0, 0));
    assert_eq!(xkb_masks(u16::MAX), (1 | 4 | 8 | 64 | 128, 2 | 16));
}
