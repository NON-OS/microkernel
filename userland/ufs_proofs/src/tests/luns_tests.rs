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

//! The REPORT LUNS list as the driver reads it: single-level LUNs only,
//! each once, ascending, at most eight, and a short list read as short.

use crate::ufs::scsi::luns::{parse, MAX_UNITS};

fn list(entries: &[[u8; 8]], claimed: u32) -> Vec<u8> {
    let mut b = claimed.to_be_bytes().to_vec();
    b.extend([0; 4]);
    entries.iter().for_each(|e| b.extend(e));
    b
}

fn lun(n: u8) -> [u8; 8] {
    [0, n, 0, 0, 0, 0, 0, 0]
}

#[test]
fn units_are_kept_once_in_order_and_well_known_or_odd_entries_dropped() {
    let entries = [
        lun(5),
        lun(0),
        lun(5),
        [0xC1, 0x01, 0, 0, 0, 0, 0, 0],
        [0, 2, 0, 1, 0, 0, 0, 0],
        lun(0x81),
        lun(2),
    ];
    let (luns, n) = parse(&list(&entries, 7 * 8));
    assert_eq!(&luns[..n], &[0, 2, 5]);
}

#[test]
fn at_most_eight_and_no_more_than_the_list_claims() {
    let entries: Vec<[u8; 8]> = (0..12).map(lun).collect();
    let (_, n) = parse(&list(&entries, 12 * 8));
    assert_eq!(n, MAX_UNITS);
    let (luns, n) = parse(&list(&entries, 2 * 8));
    assert_eq!(&luns[..n], &[0, 1], "the length field bounds the entries");
    assert_eq!(parse(&[0, 0, 0]).1, 0, "a header that is cut");
    assert_eq!(parse(&list(&[lun(1)], 64)).1, 1, "a length past the data");
}
