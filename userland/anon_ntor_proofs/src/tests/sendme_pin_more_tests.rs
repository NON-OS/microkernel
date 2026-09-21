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
//! Sendme pin, continued.

use crate::circuit::window::{circuit_sendme_due, CIRCUIT_INCREMENT};

fn digest_of(cell: i32) -> [u8; 20] {
    let mut d = [0u8; 20];
    d[..4].copy_from_slice(&cell.to_be_bytes());
    d
}

fn pin_over(cells: i32) -> (i32, Option<[u8; 20]>) {
    let mut delivered = 0i32;
    let mut owed: Option<[u8; 20]> = None;
    for cell in 1..=cells {
        delivered += 1;
        if circuit_sendme_due(delivered) && owed.is_none() {
            owed = Some(digest_of(cell));
        }
    }
    (delivered, owed)
}

#[test]
fn later_cells_do_not_move_the_pin() {
    let (delivered, owed) = pin_over(CIRCUIT_INCREMENT + 37);
    assert_eq!(delivered, 137);
    assert_eq!(owed, Some(digest_of(100)));
    assert_ne!(owed, Some(digest_of(137)), "reading the digest at send time is the failure");
}
#[test]
fn the_next_window_pins_its_own_cell() {
    // Picking up where the previous test left off: paid, so the pin is cleared
    // and the increment is taken off the count.
    let mut owed: Option<[u8; 20]> = None;
    let mut delivered = CIRCUIT_INCREMENT + 37 - CIRCUIT_INCREMENT;
    assert_eq!(delivered, 37, "the surplus carries over rather than being discarded");
    for cell in 138..=200 {
        delivered += 1;
        if circuit_sendme_due(delivered) && owed.is_none() {
            owed = Some(digest_of(cell));
        }
    }
    assert_eq!(owed, Some(digest_of(200)), "the cell that brought the carried count to a hundred");
}
