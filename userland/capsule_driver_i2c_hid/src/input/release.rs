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

//! Letting go of what the pad holds down. A pad that resets or is put to
//! sleep forgets its click button and its fingers; the router heard the
//! button go down and has to hear it come up, or it stays held.

use nonos_libc::INPUT_KIND_BUTTON_UP;

use super::post::post;
use super::TouchGesture;
use crate::state::{FrameRepeat, State};

pub fn release_held(state: &mut State) {
    let mut ok = true;
    if state.gesture.was_button {
        ok &= post(INPUT_KIND_BUTTON_UP, 1, 0, 0);
    }
    for bit in 0..5u8 {
        if state.last_buttons & (1 << bit) != 0 {
            ok &= post(INPUT_KIND_BUTTON_UP, u32::from(bit) + 1, 0, 0);
        }
    }
    if !ok {
        state.post_failures = state.post_failures.wrapping_add(1);
    }
    state.last_buttons = 0;
    state.gesture = TouchGesture::default();
    state.repeat = FrameRepeat::default();
}

/// Let go of the click the gesture holds, and of nothing else: a button the
/// pad's mouse reports hold down is theirs to let go of, button 1 included.
pub fn release_touch_click(state: &mut State) {
    if state.last_buttons & 1 == 0 && !post(INPUT_KIND_BUTTON_UP, 1, 0, 0) {
        state.post_failures = state.post_failures.wrapping_add(1);
    }
    state.gesture = TouchGesture::default();
}
