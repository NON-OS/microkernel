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

//! The reservations' wire: what the capsule encodes the kernel decodes the
//! same, and a list with any record that names no bytes, wraps, runs past
//! port 0xFFFF, names another space or sets a reserved byte is refused.

use super::wire::*;

fn r(space: u8, base: u64, len: u64) -> Reservation {
    Reservation { space, base, len }
}

#[test]
fn round_trip() {
    let list = [r(MEMORY, 0xE000_0000, 0x1000_0000), r(IO, 0xFFF0, 0x10)];
    let b = encode(&list).expect("encodes");
    assert_eq!(b.len(), 2 * RECORD_LEN);
    assert_eq!(decode(&b).unwrap(), list);
    assert_eq!(encode(&[]), Some(vec![]), "an empty _CRS publishes nothing");
}

#[test]
fn bad_records_refuse_the_list() {
    let good = r(MEMORY, 0x1000, 0x1000);
    for bad in [
        r(MEMORY, 0x1000, 0),
        r(MEMORY, u64::MAX, 2),
        r(IO, 0xFFF0, 0x11),
        r(0, 0x1000, 0x10),
        r(3, 0x1000, 0x10),
    ] {
        assert_eq!(encode(&[good, bad]), None, "{bad:?}");
    }
    let mut b = encode(&[good]).unwrap();
    for at in 1..8 {
        b[at] = 1;
        assert_eq!(decode(&b), None);
        b[at] = 0;
    }
}

#[test]
fn lengths_off_the_record_are_refused() {
    let b = encode(&[r(IO, 0x60, 1)]).unwrap();
    assert_eq!(decode(&b[..RECORD_LEN - 1]), None);
    assert_eq!(decode(&vec![0; (MAX_RANGES + 1) * RECORD_LEN]), None);
    let most = encode(&vec![r(IO, 0x60, 1); MAX_RANGES]).unwrap();
    assert_eq!(decode(&most).map(|l| l.len()), Some(MAX_RANGES));
}
