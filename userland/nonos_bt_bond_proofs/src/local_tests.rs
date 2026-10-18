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

//! This host's own IRK as kept: version 1 and the key, nothing else.

use nonos_bt_bond::{BondError, LocalIrk, LOCAL_LEN};

const KEY: [u8; 16] = *b"this-hosts-irk!!";

#[test]
fn the_key_round_trips_in_seventeen_octets() {
    let k = LocalIrk::new(KEY).unwrap();
    let b = k.encode();
    assert_eq!(b.len(), LOCAL_LEN);
    assert_eq!((b[0], &b[1..]), (1, &KEY[..]));
    assert_eq!(LocalIrk::decode(&b), Ok(k));
}

#[test]
fn a_zero_key_and_a_wrong_record_are_refused() {
    assert_eq!(LocalIrk::new([0; 16]), Err(BondError::Invalid));
    let b = LocalIrk::new(KEY).unwrap().encode();
    assert_eq!(LocalIrk::decode(&b[..16]), Err(BondError::Length));
    assert_eq!(LocalIrk::decode(&[&b[..], &[0]].concat()), Err(BondError::Length));
    let mut v = b;
    v[0] = 2;
    assert_eq!(LocalIrk::decode(&v), Err(BondError::Version));
    let mut z = [0u8; LOCAL_LEN];
    z[0] = 1;
    assert_eq!(LocalIrk::decode(&z), Err(BondError::Invalid));
}
