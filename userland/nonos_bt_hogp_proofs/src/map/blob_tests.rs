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

//! The end of a long map, as servers may answer it (Vol 3, Part F,
//! 3.4.4.5): an empty Read Blob Response, Attribute Not Long or Invalid
//! Offset all end it; any other error leaves boot mode.

use super::fallback_tests::boot;
use super::maps::unnumbered;
use crate::sim::report_devices::*;
use crate::sim::{Peripheral, Run};
use nonos_bt_hogp::report::Input::*;
use nonos_bt_wire::att::error::{ATTRIBUTE_NOT_LONG, INVALID_OFFSET, UNLIKELY_ERROR};
use nonos_bt_wire::att::opcode::READ_BLOB_REQ;

/// The unnumbered keyboard, padded with items that change nothing to
/// three full responses.
fn peer(blob_end: Option<u8>) -> Peripheral {
    let mut map = unnumbered();
    map.resize(66, 0x34);
    let mut p = Peripheral::new(report_keyboard_with(&map, [&[0, 1], &[9, 1], &[0, 2]]));
    p.blob_end = blob_end;
    p
}

#[test]
fn each_end_of_a_map_that_fills_its_responses() {
    for end in [None, Some(ATTRIBUTE_NOT_LONG), Some(INVALID_OFFSET)] {
        let mut run = Run::with(peer(end));
        ready(&mut run);
        assert_eq!(run.sent.iter().filter(|p| p[0] == 0x0c).count(), 3);
        assert_eq!(run.peer.writes[0], (MODE, vec![1], true));
        assert_eq!(
            run.notify(KEYS_IN, &[0x01, 0, 0x04, 0, 0, 0, 0, 0]),
            [KeyDown(0xe0), KeyDown(0x04)]
        );
        assert_eq!(run.notify(CONSUMER_IN, &[0x04, 0]), [], "report 9 was passed over");
    }
}

#[test]
fn a_response_one_short_of_full_ends_the_map() {
    let mut map = unnumbered();
    map.resize(43, 0x34);
    let mut run = Run::new(report_keyboard_with(&map, [&[0, 1], &[9, 1], &[0, 2]]));
    ready(&mut run);
    assert_eq!(run.sent.iter().filter(|p| p[0] == 0x0c).count(), 1, "22 then 21 octets");
}

#[test]
fn a_map_cut_off_by_an_error_is_not_used() {
    let mut p = peer(None);
    p.refuse_op = Some((READ_BLOB_REQ, UNLIKELY_ERROR));
    boot(&mut Run::with(p));
}
