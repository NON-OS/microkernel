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
use super::active;
use super::keypad::character;
use super::modifiers::{MOD_ALTGR, MOD_CAPS, MOD_SHIFT};
use super::translate::Translated;
use nonos_keymap::HeldKeys;
use nonos_libc::{mk_input_event_post, InputEvent, INPUT_KIND_KEY_DOWN, INPUT_KIND_KEY_UP};

pub fn publish(t: Translated, mods: u16, caps: bool, held: &mut HeldKeys) -> bool {
    // Printable keys carry their final character: the US base from the
    // scancode table resolved through the active layout, shift and caps
    // state, so shift-minus arrives as '_' and shift-a as 'A'. Navigation,
    // function and modifier keycodes live above the ASCII range and pass
    // through. A key the active layout leaves empty, such as the ISO key on a
    // US layout, produces nothing rather than a null character.
    // A keypad key keeps its own code (keypad.rs says why).
    let resolved =
        character(&t, mods & MOD_SHIFT != 0, caps, mods & MOD_ALTGR != 0, active::current());
    // A release carries the code its press went down with, not one resolved
    // again under the modifiers of the moment: Shift+A let go after Shift was
    // 'a', which the router could not pair with its press and a Wayland
    // client held, repeating it, for good.
    let posts = held.event(t.keycode, t.is_release, resolved);
    let flags = if caps { mods | MOD_CAPS } else { mods };
    let mut ok = true;
    if let Some(code) = posts.release_first {
        ok &= post(INPUT_KIND_KEY_UP, flags, code);
    }
    let Some(code) = posts.code else { return false };
    let kind = if t.is_release { INPUT_KIND_KEY_UP } else { INPUT_KIND_KEY_DOWN };
    ok & post(kind, flags, code)
}

fn post(kind: u16, flags: u16, code: u32) -> bool {
    let ev = InputEvent { kind, flags, code, x: 0, y: 0, delta_x: 0, delta_y: 0, timestamp_ns: 0 };
    mk_input_event_post(&ev) >= 0
}
