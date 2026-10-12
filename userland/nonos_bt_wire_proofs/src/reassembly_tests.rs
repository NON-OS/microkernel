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

//! Joining ACL fragments into L2CAP PDUs (Vol 3, Part A, 7.2).

use nonos_bt_wire::hci::Boundary::{Continuing as C, FirstFlushable as F};
use nonos_bt_wire::l2cap::Reassembler;
use nonos_bt_wire::WireError;

#[test]
fn fragments_join_into_one_pdu() {
    let mut r = Reassembler::new(64);
    assert_eq!(r.push(F, &[5, 0, 4, 0, 1]), Ok(None));
    assert_eq!(r.push(C, &[2, 3]), Ok(None));
    assert_eq!(r.push(C, &[4, 5]), Ok(Some(vec![5, 0, 4, 0, 1, 2, 3, 4, 5])));
    assert_eq!(r.push(F, &[0, 0, 4, 0]), Ok(Some(vec![0, 0, 4, 0])));
}

#[test]
fn a_start_header_split_across_fragments_still_joins() {
    let mut r = Reassembler::new(64);
    assert_eq!(r.push(F, &[1]), Ok(None));
    assert_eq!(r.push(C, &[0, 6, 0, 9]), Ok(Some(vec![1, 0, 6, 0, 9])));
}

#[test]
fn out_of_order_oversized_and_overlong_fragments_are_refused() {
    let mut r = Reassembler::new(8);
    assert_eq!(r.push(C, &[1]), Err(WireError::Sequence));
    assert_eq!(r.push(F, &[5, 0, 4, 0]), Err(WireError::TooLong));
    assert_eq!(r.push(C, &[1]), Err(WireError::Sequence));
    assert_eq!(r.push(F, &[1, 0, 4, 0, 1, 2]), Err(WireError::Length));
    assert_eq!(r.push(F, &[1, 0, 4, 0, 7]), Ok(Some(vec![1, 0, 4, 0, 7])));
}

#[test]
fn a_start_over_an_unfinished_pdu_drops_it_and_keeps_the_new_one() {
    let mut r = Reassembler::new(64);
    assert_eq!(r.push(F, &[9, 0, 4, 0, 1]), Ok(None));
    assert_eq!(r.push(F, &[1, 0, 6, 0, 2]), Ok(Some(vec![1, 0, 6, 0, 2])));
    assert_eq!(r.dropped(), 1);
}
