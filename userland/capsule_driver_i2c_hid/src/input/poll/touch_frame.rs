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

use crate::hid::decode_touch;
use crate::input::publish_touch::publish_touch;
use crate::state::State;

// How many quiet polls (~2ms each) after the last decoded touch frame before
// the boot-mouse fallback reopens for unmatched report ids.
const TOUCH_GATE_POLLS: u32 = 1024;

/// An absolute touchpad decodes through the parsed field map and the gesture
/// engine. True when the frame was used up here, false when it falls through
/// to the relative decode.
pub(super) fn touch_frame(state: &mut State, buf: &[u8], n: usize) -> bool {
    // The report opens with a 2-byte total length covering the prefix and
    // body. Reject a prefix that is too small to hold itself or larger than
    // what was actually read, and skip the poll instead of slicing wild.
    let total = u16::from_le_bytes([buf[0], buf[1]]) as usize;
    if total < 2 || total > n {
        return true;
    }
    let body = &buf[2..total];
    if let Some(s) = decode_touch(body, &state.touch_layout) {
        state.touch_decoded = true;
        state.polls_since_touch = 0;
        let act = state.gesture.on_touch(&s);
        publish_touch(state, &act);
        return true;
    }
    // While the touch decoder is proving itself on this device, unmatched
    // frames (vendor report ids, torn reads) are dropped rather than
    // reinterpreted by the boot-mouse heuristic as random ±127 deltas.
    // The gate decays: if the absolute stream goes quiet (mode changed,
    // device reset to its mouse collection), the relative fallback
    // reopens instead of muting the pad forever.
    state.polls_since_touch = state.polls_since_touch.saturating_add(1);
    state.touch_decoded && state.polls_since_touch < TOUCH_GATE_POLLS
}
