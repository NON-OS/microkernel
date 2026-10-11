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

//! One key onto the kernel ring: the US base resolved through the active
//! layout, shift, caps and AltGr (a keypad key keeps its own code), with a
//! release carrying the code its press went down with.

use nonos_libc::{INPUT_KIND_KEY_DOWN, INPUT_KIND_KEY_UP};

use super::codes::is_keypad;
use super::modifiers::{MOD_ALTGR, MOD_CAPS, MOD_SHIFT};
use super::{active, state::Keyboard};
use crate::post::send;

pub fn publish(kb: &mut Keyboard, code: u16, keycode: u32, release: bool) {
    let resolved = if is_keypad(code) {
        keycode
    } else {
        let (shift, altgr) = (kb.mods & MOD_SHIFT != 0, kb.mods & MOD_ALTGR != 0);
        nonos_keymap::resolve(keycode, shift, kb.caps, altgr, active::current())
    };
    let posts = kb.held.event(u32::from(code), release, resolved);
    let flags = if kb.caps { kb.mods | MOD_CAPS } else { kb.mods };
    if let Some(code) = posts.release_first {
        send(INPUT_KIND_KEY_UP, flags, code, (0, 0), (0, 0));
    }
    if let Some(code) = posts.code {
        let kind = if release { INPUT_KIND_KEY_UP } else { INPUT_KIND_KEY_DOWN };
        send(kind, flags, code, (0, 0), (0, 0));
    }
}
