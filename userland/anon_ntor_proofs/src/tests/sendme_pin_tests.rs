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
//! Sendme pin.

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
fn nothing_is_owed_before_the_increment() {
    let (delivered, owed) = pin_over(CIRCUIT_INCREMENT - 1);
    assert_eq!(delivered, 99);
    assert!(owed.is_none(), "a SENDME before the window falls due is one the far end never asked");
}
#[test]
fn the_pin_is_the_cell_that_brought_the_count_due() {
    let (_, owed) = pin_over(CIRCUIT_INCREMENT);
    assert_eq!(
        owed,
        Some(digest_of(CIRCUIT_INCREMENT)),
        "the hundredth cell, not the ninety ninth"
    );
}
