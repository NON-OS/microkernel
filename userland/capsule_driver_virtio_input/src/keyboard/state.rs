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

//! The keyboard as the PS/2 driver keeps it: modifiers, Caps Lock, and each
//! held key, named by its Linux code (keypad 1 and the main row's 1 share a
//! keycode, not a key), with the code it went down with.

use nonos_keymap::HeldKeys;

use super::codes::{keycode_for, KEYCODE_CAPS, KEYCODE_MUTE, KEYCODE_POWER};
use super::modifiers::{modifier_bit, modifier_of, MOD_ALT, MOD_CTRL};
use super::{active, publish};

pub struct Keyboard {
    pub mods: u16,
    pub caps: bool,
    pub held: HeldKeys,
}

impl Keyboard {
    pub const fn new() -> Self {
        Self { mods: 0, caps: false, held: HeldKeys::new() }
    }

    /// A Linux key code and 0 release, 1 press, 2 repeat; mute and power act once.
    pub fn key(&mut self, code: u16, value: i32) {
        let Some(keycode) = keycode_for(code) else { return };
        let (key, release) = (u32::from(code), value == 0);
        let repeat = !release && (value == 2 || self.held.code(key).is_some());
        if repeat && matches!(keycode, KEYCODE_MUTE | KEYCODE_POWER) {
            return;
        }
        if let Some(bit) = modifier_bit(keycode) {
            if !release {
                self.mods |= bit;
            } else if !self.held.other_held(key, |k| modifier_of(k) == Some(bit)) {
                // Both Shifts share a bit: letting one go keeps it while the
                // other is down.
                self.mods &= !bit;
            }
        }
        if !release && !repeat && keycode == KEYCODE_CAPS {
            self.caps = !self.caps;
        }
        // Ctrl+Alt+Space cycles the layout here and is consumed, as on PS/2.
        let chord = self.mods & MOD_CTRL != 0 && self.mods & MOD_ALT != 0;
        if !release && keycode == b' ' as u32 && chord {
            if !repeat {
                let _ = active::cycle();
                let _ = self.held.press(key, 0);
            }
            return;
        }
        publish::publish(self, code, keycode, release);
    }
}

impl Default for Keyboard {
    fn default() -> Self {
        Self::new()
    }
}
