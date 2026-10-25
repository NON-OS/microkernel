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

//! The registry is append-only and global, so its whole life is one test:
//! publish, refuse the empty and the overlapping, look up by (segment, bus),
//! list each segment once, and refuse once every slot is taken.

use super::registry::{lookup, publish, segments};
use super::{EcamError, EcamWindow};

const fn win(segment: u16, start_bus: u8, end_bus: u8) -> EcamWindow {
    EcamWindow { segment, start_bus, end_bus, va: 0x1000_0000 * (segment as u64 + 1) }
}

#[test]
fn the_registry_from_empty_to_full() {
    assert_eq!(lookup(0, 0), None);
    assert_eq!(segments().count(), 0);

    assert_eq!(publish(win(0, 5, 4)), Err(EcamError::Empty));
    assert_eq!(publish(EcamWindow { va: 0, ..win(0, 0, 7) }), Err(EcamError::Empty));

    assert_eq!(publish(win(0, 0, 7)), Ok(()));
    assert_eq!(publish(win(0, 7, 9)), Err(EcamError::Overlaps));
    assert_eq!(publish(win(0, 0, 0)), Err(EcamError::Overlaps));
    assert_eq!(publish(win(0, 8, 15)), Ok(()));
    assert_eq!(publish(win(0x1000, 0, 0)), Ok(()));

    assert_eq!(lookup(0, 3), Some(win(0, 0, 7)));
    assert_eq!(lookup(0, 15), Some(win(0, 8, 15)));
    assert_eq!(lookup(0, 16), None);
    assert_eq!(lookup(0x1000, 0), Some(win(0x1000, 0, 0)));
    assert_eq!(lookup(0x1000, 1), None);
    assert_eq!(lookup(1, 0), None);
    assert_eq!(segments().collect::<Vec<_>>(), [0, 0x1000]);

    // Three are in; fill the rest one segment each, then one more.
    let mut published = 3;
    let mut seg = 1;
    while publish(win(seg, 0, 255)).is_ok() {
        published += 1;
        seg += 1;
    }
    assert_eq!(published, 32);
    assert_eq!(publish(win(0x2000, 0, 0)), Err(EcamError::Full));
    assert_eq!(segments().count(), 31);
    assert_eq!(lookup(seg - 1, 200), Some(win(seg - 1, 0, 255)));
    assert_eq!(lookup(0, 3), Some(win(0, 0, 7)));
}
