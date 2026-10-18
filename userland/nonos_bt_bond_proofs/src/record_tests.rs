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

//! A bond record octet by octet, as `layout.rs` sets it out, and back.

use crate::fixture::{public, with_irk, IRK};
use nonos_bt_bond::{BondRecord, RECORD_LEN};

#[test]
fn a_bond_with_an_irk_is_laid_out_as_documented() {
    let b = with_irk().encode().unwrap();
    assert_eq!(b.len(), RECORD_LEN);
    assert_eq!(b[..3], [1, 0x07, 1]);
    assert_eq!(b[3..9], [0x10, 0x20, 0x30, 0x40, 0x50, 0xc0]);
    assert_eq!(b[9..25], *b"long-term-key-16");
    assert_eq!(b[25..41], IRK);
    assert_eq!(b[41], 16);
    assert_eq!(b[42..], [0; 6]);
    assert_eq!(BondRecord::decode(&b), Ok(with_irk()));
}

#[test]
fn a_bond_without_an_irk_has_its_flag_clear_and_its_field_zero() {
    let b = public().encode().unwrap();
    assert_eq!(b[..3], [1, 0x04, 0]);
    assert_eq!(b[25..41], [0; 16]);
    assert_eq!(BondRecord::decode(&b), Ok(public()));
}

#[test]
fn a_record_that_breaks_a_rule_is_not_written() {
    let bad = [
        BondRecord { key_size: 6, ..with_irk() },
        BondRecord { key_size: 17, ..with_irk() },
        BondRecord { ltk: [0; 16], ..with_irk() },
        BondRecord { irk: Some([0; 16]), ..with_irk() },
        BondRecord { identity_type: 2, ..with_irk() },
        BondRecord { identity: [1, 2, 3, 4, 5, 0x80], ..with_irk() },
        BondRecord { identity: [0, 0, 0, 0, 0, 0xc0], ..with_irk() },
        BondRecord { identity: [0xff; 6], ..with_irk() },
    ];
    for r in bad {
        assert!(r.encode().is_err(), "{r:?}");
    }
}
