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

//! The modifier bits, and the one chord the driver keeps for itself.

use super::drainer::Drainer;
use crate::keymap;
use crate::keymap::translate::Translated;

/// Set or clear the modifier bit `t` holds. Both Shifts (or Ctrls, or
/// Metas) share a bit: letting one go while the other is down leaves it set.
pub(super) fn track_modifiers(drainer: &mut Drainer, t: &Translated) {
    let Some(bit) = keymap::modifier_bit(t.keycode) else { return };
    if !t.is_release {
        drainer.mods |= bit;
    } else if !drainer.held.other_held(t.keycode, |k| keymap::modifier_bit(k) == Some(bit)) {
        drainer.mods &= !bit;
    }
}

/// Ctrl+Alt+Space cycles the keyboard layout inside the driver and is
/// consumed here; no app has a use for that chord as input. True when `t`
/// was that chord's Space.
pub(super) fn layout_chord(drainer: &mut Drainer, t: &Translated, repeat: bool) -> bool {
    let both = keymap::MOD_CTRL | keymap::MOD_ALT;
    if t.is_release || t.keycode != b' ' as u32 || drainer.mods & both != both {
        return false;
    }
    if !repeat {
        let _ = keymap::active::cycle();
        // Held with no code: its release posts nothing either.
        let _ = drainer.held.press(t.keycode, 0);
    }
    true
}
