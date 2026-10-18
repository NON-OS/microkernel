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

//! The L2CAP basic frame (Vol 3, Part A, 3.1) and LE fixed channel IDs
//! (Vol 3, Part A, 2.1).

use nonos_bt_wire::l2cap::{basic, cid, parse_basic};
use nonos_bt_wire::WireError;

#[test]
fn basic_frames_round_trip() {
    let f = basic(cid::ATT, &[0x02, 0x17, 0x00]).unwrap();
    assert_eq!(f, [3, 0, 4, 0, 0x02, 0x17, 0x00]);
    assert_eq!(parse_basic(&f), Ok((cid::ATT, &[0x02u8, 0x17, 0x00][..])));
    assert_eq!(parse_basic(&f[..6]), Err(WireError::Short));
    assert_eq!(parse_basic(&[f.as_slice(), &[0]].concat()), Err(WireError::Length));
}

#[test]
fn le_fixed_channels() {
    assert_eq!((cid::ATT, cid::LE_SIGNALING, cid::SMP), (4, 5, 6));
    assert!(cid::LE_DYNAMIC.contains(&0x40) && !cid::LE_DYNAMIC.contains(&0x80));
}
