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

use super::read_frame::read_frame;
use super::repeat::skip_repeat;
use super::touch_frame::touch_frame;
use crate::hid::decode_mouse;
use crate::input::parse_report::parse_report;
use crate::input::publish::publish;
use crate::state::{State, FRAME_MAX};

pub fn poll(state: &mut State) {
    if !state.found() || state.input_register == 0 {
        return;
    }
    // A full Precision Touchpad report carries several contacts plus the
    // contact-count and scan-time trailer and can run past 64 bytes; the
    // length prefix at the head still drives how much of this we parse.
    let mut buf = [0u8; FRAME_MAX];
    let Some(n) = read_frame(state, &mut buf) else {
        return;
    };
    if skip_repeat(&mut state.repeat, &buf, n) {
        return;
    }
    // An absolute touchpad decodes through the parsed field map and the gesture
    // engine; anything else falls back to the relative boot-mouse decode.
    if state.touch_layout.is_absolute_touch() && touch_frame(state, &buf, n) {
        return;
    }
    // The mouse collection, decoded at the positions the descriptor gives
    // when it declares one; the boot-mouse guess is kept for a descriptor
    // that could not be read.
    if state.mouse_layout.is_relative_mouse() {
        let total = u16::from_le_bytes([buf[0], buf[1]]) as usize;
        if (3..=n).contains(&total) {
            if let Some(sample) = decode_mouse(&buf[2..total], &state.mouse_layout) {
                publish(state, sample);
            }
        }
        return;
    }
    if let Some(sample) = parse_report(&buf[..n]) {
        publish(state, sample);
    }
}
