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

/// Pay every SENDME owed, oldest first, as `out/sendme_tick.rs` does.
fn pay_all(delivered: &mut i32, owed: &mut Vec<[u8; 20]>) -> Vec<[u8; 20]> {
    let paid = std::mem::take(owed);
    *delivered -= CIRCUIT_INCREMENT * paid.len() as i32;
    paid
}

#[test]
fn later_cells_do_not_move_the_pin() {
    let (mut delivered, mut owed) = (0, Vec::new());
    deliver(&mut delivered, &mut owed, 1, 137);
    assert_eq!(owed, vec![digest_of(100)]);
    assert_ne!(owed[0], digest_of(137), "reading the digest at send time is the failure");
    let paid = pay_all(&mut delivered, &mut owed);
    assert_eq!((paid, delivered), (vec![digest_of(100)], 37), "the surplus carries over");
    deliver(&mut delivered, &mut owed, 138, 200);
    assert_eq!(owed, vec![digest_of(200)], "the cell that brought the carried count to a hundred");
}

#[test]
fn a_sendme_held_back_is_not_overtaken_by_the_next_boundary() {
    // The model download: the reader is behind, so nothing is paid while
    // three hundred and forty cells arrive. One slot pinned cell 100, then,
    // after paying, pinned cell 341 for the second SENDME where the exit had
    // recorded cell 200, and the exit destroyed the circuit.
    let (mut delivered, mut owed) = (0, Vec::new());
    deliver(&mut delivered, &mut owed, 1, 340);
    assert_eq!(owed, vec![digest_of(100), digest_of(200), digest_of(300)]);
    let paid = pay_all(&mut delivered, &mut owed);
    assert_eq!(paid, vec![digest_of(100), digest_of(200), digest_of(300)], "oldest first");
    assert_eq!(delivered, 40);
    deliver(&mut delivered, &mut owed, 341, 400);
    assert_eq!(owed, vec![digest_of(400)], "and the boundaries stay where the exit counts them");
}
