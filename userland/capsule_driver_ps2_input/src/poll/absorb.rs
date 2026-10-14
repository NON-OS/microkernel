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

use super::chord::{layout_chord, track_modifiers};
use super::counters::Counters;
use super::drainer::Drainer;
use super::prefix::Scan;
use crate::keymap;

/// One byte from the keyboard: framed by its prefixes, decoded, folded into
/// the modifier and lock state, and posted. Nothing of it is kept once it is
/// posted but the code of a key still held, which its release carries.
pub fn absorb(drainer: &mut Drainer, counters: &mut Counters, byte: u8) {
    counters.seen = counters.seen.wrapping_add(1);
    let t = match drainer.prefix.take(byte) {
        Scan::Key { scancode, flags } => keymap::translate(scancode, flags),
        Scan::Reply(reply) => {
            drainer.reply = Some(reply);
            None
        }
        Scan::Pending => None,
    };
    let Some(t) = t else { return };
    // The keyboard repeats a held key as more make codes. A repeat must
    // not act like a fresh press where a press acts once: Caps Lock held
    // flickered caps at the repeat rate, and Ctrl+Alt+Space held cycled
    // through every layout.
    let repeat = !t.is_release && drainer.held.code(t.keycode).is_some();
    if repeat && keymap::acts_once(t.keycode) {
        return;
    }
    track_modifiers(drainer, &t);
    if !t.is_release && !repeat {
        if let Some(bit) = keymap::lock_bit(t.keycode) {
            drainer.locks ^= bit;
        }
    }
    if layout_chord(drainer, &t, repeat) {
        return;
    }
    let caps = drainer.caps();
    if !keymap::publish(t, drainer.mods, caps, &mut drainer.held) {
        counters.refused = counters.refused.wrapping_add(1);
    }
}
