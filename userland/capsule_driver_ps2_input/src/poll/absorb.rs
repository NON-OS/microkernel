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
use super::drainer::Drainer;
use crate::keymap;
use crate::ring::{Event, Ring, FLAG_BREAK, FLAG_E0_PREFIX, FLAG_E1_PREFIX};
const E0_PREFIX: u8 = 0xE0;
const E1_PREFIX: u8 = 0xE1;
const BREAK_BIT: u8 = 0x80;
pub(super) fn absorb(drainer: &mut Drainer, ring: &mut Ring, byte: u8) {
    if byte == E0_PREFIX {
        drainer.pending_e0 = true;
        return;
    }
    if byte == E1_PREFIX {
        drainer.pending_e1 = true;
        return;
    }
    let mut flags: u8 = 0;
    if byte & BREAK_BIT != 0 {
        flags |= FLAG_BREAK;
    }
    if drainer.pending_e0 {
        flags |= FLAG_E0_PREFIX;
        drainer.pending_e0 = false;
    }
    if drainer.pending_e1 {
        flags |= FLAG_E1_PREFIX;
        drainer.pending_e1 = false;
    }
    ring.push(Event { scancode: byte, flags });
    if let Some(t) = keymap::translate(byte, flags) {
        // The keyboard repeats a held key as more make codes. A repeat must
        // not act like a fresh press where a press acts once: Caps Lock held
        // flickered caps at the repeat rate, and Ctrl+Alt+Space held cycled
        // through every layout.
        let repeat = !t.is_release && drainer.held.code(t.keycode).is_some();
        if repeat && keymap::acts_once(t.keycode) {
            return;
        }
        if let Some(bit) = keymap::modifier_bit(t.keycode) {
            if !t.is_release {
                drainer.mods |= bit;
            } else if !drainer.held.other_held(t.keycode, |k| keymap::modifier_bit(k) == Some(bit))
            {
                // Both Shifts (or Ctrls, or Metas) share a bit: letting one go
                // while the other is down must leave it set.
                drainer.mods &= !bit;
            }
        }
        if !t.is_release && !repeat && t.keycode == keymap::KEYCODE_CAPS_LOCK {
            drainer.caps = !drainer.caps;
        }
        // Ctrl+Alt+Space cycles the keyboard layout inside the driver and
        // is consumed here; no app has a use for that chord as input.
        if !t.is_release
            && t.keycode == b' ' as u32
            && drainer.mods & keymap::MOD_CTRL != 0
            && drainer.mods & keymap::MOD_ALT != 0
        {
            if !repeat {
                let _ = keymap::active::cycle();
                // Held with no code: its release posts nothing either.
                let _ = drainer.held.press(t.keycode, 0);
            }
            return;
        }
        let _ = keymap::publish(t, drainer.mods, drainer.caps, &mut drainer.held);
    }
}
