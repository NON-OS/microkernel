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

//! The modifier state a key event carries (app_skeleton input/modifiers.rs)
//! as the xkb masks wl_keyboard.modifiers sends. Pure, so the host proofs
//! hold it.
//!
//! The core modifiers come first in every xkb keymap, in a fixed order:
//! Shift, Lock, Control, Mod1 to Mod5. Alt is Mod1, Num Lock Mod2, the logo
//! key Mod4, and AltGr (ISO_Level3_Shift) Mod5.

pub const SHIFT: u16 = 1 << 0;
pub const CTRL: u16 = 1 << 1;
pub const ALT: u16 = 1 << 2;
pub const META: u16 = 1 << 3;
pub const CAPS: u16 = 1 << 4;
pub const NUM: u16 = 1 << 5;
pub const ALTGR: u16 = 1 << 6;

/// (depressed, locked): what is held, and the locks that are on.
pub fn xkb_masks(flags: u16) -> (u32, u32) {
    let bit = |flag: u16, mask: u32| if flags & flag != 0 { mask } else { 0 };
    let depressed = bit(SHIFT, 1)
        | bit(CTRL, 1 << 2)
        | bit(ALT, 1 << 3)
        | bit(META, 1 << 6)
        | bit(ALTGR, 1 << 7);
    let locked = bit(CAPS, 1 << 1) | bit(NUM, 1 << 4);
    (depressed, locked)
}
