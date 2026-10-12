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

//! Every way a stored bond record can be wrong, each refused on reading.

use crate::fixture::{public, with_irk};
use nonos_bt_bond::{BondError, BondRecord};

fn changed(at: usize, v: u8) -> Result<BondRecord, BondError> {
    let mut b = with_irk().encode().unwrap();
    b[at] = v;
    BondRecord::decode(&b)
}

#[test]
fn wrong_lengths_and_versions_are_refused() {
    let b = with_irk().encode().unwrap();
    assert_eq!(BondRecord::decode(&b[..47]), Err(BondError::Length));
    assert_eq!(BondRecord::decode(&[&b[..], &[0]].concat()), Err(BondError::Length));
    assert_eq!(changed(0, 0), Err(BondError::Version));
    assert_eq!(changed(0, 2), Err(BondError::Version));
}

#[test]
fn unknown_flags_legacy_and_reserved_octets_are_refused() {
    assert_eq!(changed(1, 0x0f), Err(BondError::Invalid));
    assert_eq!(changed(1, 0x83), Err(BondError::Invalid));
    assert_eq!(changed(1, 0x03), Err(BondError::Invalid));
    for at in 42..48 {
        assert_eq!(changed(at, 1), Err(BondError::Invalid));
    }
}

#[test]
fn fields_out_of_range_are_refused() {
    assert_eq!(changed(2, 2), Err(BondError::Invalid));
    assert_eq!(changed(8, 0x80), Err(BondError::Invalid));
    assert_eq!(changed(41, 6), Err(BondError::Invalid));
    assert_eq!(changed(41, 17), Err(BondError::Invalid));
}

#[test]
fn the_irk_field_must_agree_with_its_flag() {
    let mut b = public().encode().unwrap();
    b[30] = 1;
    assert_eq!(BondRecord::decode(&b), Err(BondError::Invalid));
    let mut b = with_irk().encode().unwrap();
    b[25..41].fill(0);
    assert_eq!(BondRecord::decode(&b), Err(BondError::Invalid));
}
