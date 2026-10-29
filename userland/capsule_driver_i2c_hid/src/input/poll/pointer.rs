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

//! A report as pointer input: the touch report through the touch layout,
//! the mouse report through the mouse layout, and nothing else. Each
//! decode reads a report only when its report ID is the layout's.

use crate::drive::mouse_seen;
use crate::hid::{decode_mouse, decode_touch};
use crate::input::publish::publish;
use crate::input::publish_touch::publish_touch;
use crate::input::release_touch_click;
use crate::state::{Seen, State};

pub(super) fn pointer(state: &State, report: &[u8]) -> Option<Seen> {
    if state.touch_layout.is_absolute_touch() {
        if let Some(s) = decode_touch(report, &state.touch_layout) {
            return Some(Seen::Touch(s));
        }
    }
    if !state.mouse_layout.is_relative_mouse() {
        return None;
    }
    decode_mouse(report, &state.mouse_layout).map(Seen::Mouse)
}

/// What a report says before the repeat check, a repeat too: a still
/// finger's frames are touch reports as much as a moving one's. A touch
/// report says touchpad mode holds. On a pad whose mode did not read back,
/// a run of mouse reports with none (drive/mouse_run.rs) says it left
/// touchpad mode, and the click the gesture made is let go; where the mode
/// reads back, that read-back and the reset ring tell. The mouse reports'
/// own buttons are theirs.
pub(super) fn note(state: &mut State, seen: &Seen) {
    match seen {
        Seen::Touch(_) => state.recover.held(),
        Seen::Mouse(_) => {
            let gone = state.recover.run.mouse();
            if gone && state.recover.mode_unread && state.gesture.was_button {
                release_touch_click(state);
            }
        }
    }
}

/// Post what the sample says: a touch through the gesture engine, a mouse
/// report as it is. A pad set to touchpad mode does not report on its
/// mouse collection unless it reset, or a pointing stick does (mouse_seen).
pub(super) fn act(state: &mut State, seen: Seen) {
    match seen {
        Seen::Touch(s) => {
            let act = state.gesture.on_touch(&s);
            publish_touch(state, &act);
        }
        Seen::Mouse(sample) => {
            if state.recover.touch_mode {
                mouse_seen(state);
            }
            publish(state, sample);
        }
    }
}
