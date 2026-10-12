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

//! ATT PDUs (Vol 3, Part F, 3.4), laid out byte by byte.

use nonos_bt_wire::att::response::{error_rsp, found_ranges, handle_pairs, handle_value};
use nonos_bt_wire::att::{self, opcode::is_request, uuid16, BASE_UUID};
use nonos_bt_wire::WireError;

#[test]
fn requests_are_laid_out_little_endian() {
    let find = att::find_by_type_value(0x0001, 0xffff, 0x2800, &[0x12, 0x18]);
    assert_eq!(find, [0x06, 0x01, 0x00, 0xff, 0xff, 0x00, 0x28, 0x12, 0x18]);
    assert_eq!(
        att::read_by_type(0x0004, 0x000d, 0x2803),
        [0x08, 0x04, 0x00, 0x0d, 0x00, 0x03, 0x28]
    );
    assert_eq!(att::find_information(0x0009, 0x000a), [0x04, 0x09, 0x00, 0x0a, 0x00]);
    assert_eq!(att::read(0x0012), [0x0a, 0x12, 0x00]);
    assert_eq!(att::write(0x0009, &[1, 0], false), [0x12, 0x09, 0x00, 1, 0]);
    assert_eq!(att::write(0x0006, &[0], true), [0x52, 0x06, 0x00, 0]);
    assert_eq!(att::error_rsp(0x0a, 0, 0x06), [0x01, 0x0a, 0x00, 0x00, 0x06]);
    assert_eq!(att::confirmation(), [0x1e]);
}

#[test]
fn responses_parse_and_bad_lengths_are_refused() {
    let e = error_rsp(&[0x08, 0x04, 0x00, 0x0a]).unwrap();
    assert_eq!((e.request, e.handle, e.code), (0x08, 4, 0x0a));
    assert_eq!(error_rsp(&[0x08, 0x04, 0x00]), Err(WireError::Short));
    assert_eq!(found_ranges(&[4, 0, 13, 0]), Ok(vec![(4, 13)]));
    assert_eq!(found_ranges(&[4, 0, 13]), Err(WireError::Length));
    let pairs = handle_pairs(&[9, 0, 0x02, 0x29, 10, 0, 0x08, 0x29], 4).unwrap();
    assert_eq!(pairs, [(9, &[0x02, 0x29][..]), (10, &[0x08, 0x29][..])]);
    assert_eq!(handle_pairs(&[9, 0, 0x02], 4), Err(WireError::Length));
    assert_eq!(handle_pairs(&[9], 1), Err(WireError::Length));
    let hv = handle_value(&[8, 0, 1, 2]).unwrap();
    assert_eq!((hv.handle, hv.value), (8, &[1, 2][..]));
}

#[test]
fn uuids_and_which_opcodes_are_requests() {
    assert_eq!(uuid16(&[0x12, 0x18]), Some(0x1812));
    let mut long = BASE_UUID;
    long[12..14].copy_from_slice(&[0x02, 0x29]);
    assert_eq!(uuid16(&long), Some(0x2902));
    long[0] ^= 1;
    assert_eq!(uuid16(&long), None);
    assert_eq!(uuid16(&[1, 2, 3]), None);
    assert!(is_request(0x0a) && is_request(0x02) && is_request(0x20));
    assert!(!is_request(0x52) && !is_request(0x1b) && !is_request(0x13) && !is_request(0x01));
}
