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

use crate::circuit::window::{circuit_sendme_boundary, CIRCUIT_INCREMENT};

fn digest_of(cell: i32) -> [u8; 20] {
    let mut d = [0u8; 20];
    d[..4].copy_from_slice(&cell.to_be_bytes());
    d
}

/// Deliver cells `from..=to`, pinning as `inbound/data.rs` does.
fn deliver(delivered: &mut i32, owed: &mut Vec<[u8; 20]>, from: i32, to: i32) {
    for cell in from..=to {
        *delivered += 1;
        if circuit_sendme_boundary(*delivered) {
            owed.push(digest_of(cell));
        }
    }
}


#[test]
fn nothing_is_owed_before_the_increment() {
    let (mut delivered, mut owed) = (0, Vec::new());
    deliver(&mut delivered, &mut owed, 1, CIRCUIT_INCREMENT - 1);
    assert_eq!(delivered, 99);
    assert!(owed.is_empty(), "a SENDME before the window falls due is one the far end never asked");
}

#[test]
fn the_pin_is_the_cell_that_brought_the_count_due() {
    let (mut delivered, mut owed) = (0, Vec::new());
    deliver(&mut delivered, &mut owed, 1, CIRCUIT_INCREMENT);
    assert_eq!(owed, vec![digest_of(CIRCUIT_INCREMENT)], "the hundredth cell, not the ninety ninth");
}
