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

//! A pad that reset itself comes back in its power-on input mode, the
//! mouse, with its switches at their defaults: it is set up again and what
//! it held is let go. The specification's sign of a reset is the
//! zero-length report its line rang for (rang.rs). A mouse report from a
//! pad set to touchpad mode is one only if the pad's Input Mode reads back
//! as something else: a pointing stick or the pad's own buttons report on
//! the mouse collection too (input/poll/pointer.rs).

use crate::hid::{configure_reporting, Configured, I2cFeatures};
use crate::input::release_held;
use crate::state::State;

/// Input mode, surface and button switches set as the touch layout wants
/// them, read-modify-write, writing only a bit that differs. Touchpad mode
/// is to hold unless the layout leaves its Input Mode nothing to write (a
/// write the bus refused is tried again); whether the mode read back is
/// kept beside it.
pub fn configure(state: &mut State) -> Configured {
    let l = &state.touch_layout;
    let features = I2cFeatures { port: state.i2c_port, addr: state.addr, desc: &state.descriptor };
    let c = configure_reporting(&features, l);
    let touch_mode = l.is_absolute_touch() && l.input_mode.present() && !c.mode_unwritable;
    (state.recover.touch_mode, state.recover.mode_unread) = (touch_mode, c.mode_unread);
    c
}

/// The zero-length report the line rang for. What the pad held is let go
/// at once, as a button-up too many is harmless and a click stuck down is
/// not; the set-up is owed, within the bounds, until one takes or a touch
/// report says the mode holds.
pub fn reset_seen(state: &mut State) {
    release_held(state);
    state.recover.owed = true;
    set_up_owed(state);
}

/// The set-up a reset is owed, once the tries and the holdoff allow it.
pub(super) fn set_up_owed(state: &mut State) {
    if state.recover.owed && state.recover.may() {
        let c = configure(state);
        state.recover.ran(!c.refused, true);
    }
}

/// A mouse report from a pad set to touchpad mode. A mode the pad will not
/// read back is written again, but says nothing of a reset.
pub fn mouse_seen(state: &mut State) {
    if !state.recover.may() {
        return;
    }
    let c = configure(state);
    state.recover.ran(!c.refused, c.mode_lost);
    if c.mode_lost {
        release_held(state);
    }
}
