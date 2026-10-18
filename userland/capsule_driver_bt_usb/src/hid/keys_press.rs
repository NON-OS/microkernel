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

//! A key going down or up, and its repeat, as `capsule_driver_usb_hid`'s
//! keyboard does them, with the modifiers the Bluetooth host reports.

use super::key_event::KeyEvent;
use super::keys::Keys;
use super::modifier::layout_chord;
use super::{active, keymap, post_key};

impl Keys {
    /// The held key again when its repeat is due, as the USB driver's tick.
    pub fn tick(&mut self, now_ms: u64) {
        let Some(key) = self.repeat.due(now_ms) else { return };
        if layout_chord(self.modifiers, key) || self.held.code(u32::from(key)) == Some(0) {
            self.repeat.release(key);
            return;
        }
        let ok = self.post(key, true);
        self.failures += u64::from(!ok);
    }

    pub(super) fn press(&mut self, usage: u8) -> bool {
        self.caps ^= keymap::is_caps_lock(usage);
        if layout_chord(self.modifiers, usage) {
            let _ = active::cycle();
            // Held with no code, so its release posts nothing either.
            let _ = self.held.press(u32::from(usage), 0);
            self.repeat.release(usage);
            return true;
        }
        self.repeat.press(usage);
        self.post(usage, true)
    }

    pub(super) fn post(&mut self, usage: u8, pressed: bool) -> bool {
        let (m, caps) = (self.modifiers, self.caps);
        let ascii = if pressed { keymap::ascii(usage, m, caps) } else { 0 };
        let event = KeyEvent { scancode: usage, ascii, modifiers: m, pressed, caps };
        post_key::publish(event, &mut self.held)
    }
}
