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

//! STATE and SURFACE bodies.

use crate::body::state::{State, LOST};
use crate::body::surface::{Surface, ARGB8888};
use crate::errno::EINVAL;

#[test]
fn state_round_trips_every_field() {
    let s = State {
        generation: 7,
        width: 1920,
        height: 1080,
        serial: 0x0042_0003,
        hint_width: 2560,
        hint_height: 1440,
        flags: LOST,
    };
    assert_eq!(State::decode(&s.encode()), Ok(s));
    assert_eq!(State::decode(&s.encode()[..27]), Err(EINVAL));
}

#[test]
fn a_surface_needs_a_picture_a_row_wide_stride_and_argb() {
    let s = Surface {
        handle: 0x1122_3344_5566_7788,
        serial: 0x0042_0001,
        width: 1920,
        height: 1080,
        stride: 7680,
        format: ARGB8888,
        cursor_offset: 7680 * 1080,
    };
    assert_eq!(Surface::decode(&s.encode()), Ok(s));
    for bad in
        [Surface { stride: 7679, ..s }, Surface { format: 2, ..s }, Surface { width: 0, ..s }]
    {
        assert_eq!(Surface::decode(&bad.encode()), Err(EINVAL));
    }
}
