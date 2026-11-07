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

/// Whether the Linux key `linux` is a modifier or a lock, which xkb never
/// repeats however long it is held.
pub fn is_modifier(linux: u32) -> bool {
    matches!(linux, 29 | 42 | 54 | 56 | 58 | 69 | 97 | 100 | 125 | 126)
}

/// `masks` after the Linux key `linux` went down (`press`) or up: a
/// modifier key changes what is held at once, before the next event says
/// so. Clients read the state the last wl_keyboard.modifiers left as a key
/// arrives, so it is sent before an ordinary key, and after a modifier's.
pub fn after(masks: (u32, u32), linux: u32, press: bool) -> Option<(u32, u32)> {
    let bit = match linux {
        42 | 54 => 1,
        29 | 97 => 1 << 2,
        56 => 1 << 3,
        125 | 126 => 1 << 6,
        100 => 1 << 7,
        _ => return None,
    };
    let held = if press { masks.0 | bit } else { masks.0 & !bit };
    Some((held, masks.1))
}

/// The masks a sign is said under on the US keymap the client reads:
/// `told`, with shift as its US key needs it (evdev::us_shift) and no
/// AltGr, which that keymap gives no sign. Ctrl, Alt and the logo key are
/// kept, so Ctrl and '+' on a German layout is Ctrl, shift and '='. A key
/// with no such need is said under `told` as it is.
pub fn as_us(us_shift: Option<bool>, told: (u32, u32)) -> (u32, u32) {
    let Some(shift) = us_shift else { return told };
    let rest = told.0 & !(1 | 1 << 7);
    (if shift { rest | 1 } else { rest }, told.1)
}

/// What a key event tells the client: modifiers before the key, the key,
/// and modifiers after it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct KeySay {
    pub before: Option<(u32, u32)>,
    pub key: bool,
    pub after: Option<(u32, u32)>,
}

/// The event for the Linux key `linux` going down (`press`) or up, under
/// the masks `told`, when `news` says whether the client sees the key
/// change (held_keys.rs). A release of a modifier the client never saw go
/// down (it was held before the window had the keyboard, and an ordinary
/// key since told the client so) sends no key, but still the modifiers it
/// leaves, or the client would keep it held until the next key. An
/// ordinary key is said under `typed` (as_us), and the held masks are told
/// again after it when those differ, so a click after it is not shifted
/// the wrong way.
pub fn key_say(
    linux: u32,
    press: bool,
    news: bool,
    (told, typed): ((u32, u32), (u32, u32)),
) -> KeySay {
    let then = after(told, linux, press);
    if !news {
        return KeySay { before: None, key: false, after: then.filter(|_| !press) };
    }
    match then {
        Some(_) => KeySay { before: None, key: true, after: then },
        None => KeySay { before: Some(typed), key: true, after: (typed != told).then_some(told) },
    }
}
