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

//! CURSOR bodies, and serials.

use crate::body::cursor::{Cursor, NEW_SHAPE, VISIBLE};
use crate::errno::EINVAL;
use crate::serial::{pid_of, serial};

#[test]
fn a_cursor_round_trips_with_negative_positions() {
    let c = Cursor {
        flags: VISIBLE | NEW_SHAPE,
        x: -12,
        y: 900,
        hot_x: 3,
        hot_y: 1,
        width: 64,
        height: 64,
    };
    assert_eq!(Cursor::decode(&c.encode()), Ok(c));
}

#[test]
fn a_shape_past_64_or_a_hot_spot_outside_it_is_refused() {
    let c = Cursor { flags: VISIBLE, x: 0, y: 0, hot_x: 0, hot_y: 0, width: 32, height: 32 };
    for bad in [
        Cursor { width: 65, ..c },
        Cursor { height: 65, ..c },
        Cursor { hot_x: 32, ..c },
        Cursor { hot_y: 40, ..c },
    ] {
        assert_eq!(Cursor::decode(&bad.encode()), Err(EINVAL));
    }
    let hidden = Cursor { flags: 0, width: 0, height: 0, ..c };
    assert_eq!(Cursor::decode(&hidden.encode()), Ok(hidden), "a hidden, shapeless cursor");
    assert_eq!(Cursor::decode(&c.encode()[..19]), Err(EINVAL));
}

#[test]
fn a_serial_carries_its_drivers_pid() {
    let s = serial(0x42, 0x1_0005);
    assert_eq!((s, pid_of(s)), (0x0042_0005, 0x42));
}
