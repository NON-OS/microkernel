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

//! The codepoint a printable key resolves to under the active layout.

use super::us_base::us_base;

const SHIFT_MASK: u8 = 0x22;
// Right alt alone. On every layout here but US it is AltGr and selects a
// third level of characters rather than acting as alt.
const ALTGR_MASK: u8 = 0x40;
// Keyboard Non-US backslash: the extra key an ISO board has between the
// left shift and Z.
const ISO_KEY: u8 = 0x64;

/// Final codepoint for a printable key under the active layout, shift and
/// caps state; 0 for control, navigation and unknown usages.
pub fn resolve_code(scancode: u8, modifiers: u8, caps: bool) -> u32 {
    let shift = (modifiers & SHIFT_MASK) != 0;
    let altgr = (modifiers & ALTGR_MASK) != 0;
    let layout = super::super::active::current();
    if scancode == ISO_KEY {
        return nonos_keymap::iso(layout, shift, altgr);
    }
    let base = us_base(scancode);
    if base == 0 {
        return 0;
    }
    nonos_keymap::resolve(base as u32, shift, caps, altgr, layout)
}
