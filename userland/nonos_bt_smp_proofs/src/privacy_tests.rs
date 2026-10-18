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

//! LE privacy: ah against the sample data of Vol 3, Part H, D.7, and the
//! address it makes made and resolved as Vol 6, Part B, 1.3.2.2 and
//! 1.3.2.3 say.

use crate::hex::arr;
use nonos_bt_smp::privacy::{ah, prand, resolves, rpa};
use nonos_bt_wire::addr::{BdAddr, RandomKind};

fn irk() -> [u8; 16] {
    arr("ec0234a3 57c8ad05 341010a6 0a397d9b")
}

#[test]
fn ah_matches_the_sample_data() {
    assert_eq!(ah(&irk(), &arr("708194")), arr::<3>("0dfbaa"));
}

#[test]
fn the_sample_address_is_resolvable_and_resolves() {
    let a = rpa(&irk(), &arr("708194"));
    assert_eq!(a, BdAddr([0xaa, 0xfb, 0x0d, 0x94, 0x81, 0x70]));
    assert_eq!(a.random_kind(), Ok(RandomKind::Resolvable));
    assert!(resolves(&irk(), &a));
}

#[test]
fn prand_is_marked_resolvable_and_never_all_zero_or_all_one() {
    assert_eq!(prand([0xff, 0x12, 0x34]), Some([0x7f, 0x12, 0x34]));
    assert_eq!(prand([0x80, 0x00, 0x01]), Some([0x40, 0x00, 0x01]));
    assert_eq!(prand([0x00, 0x00, 0x00]), None);
    assert_eq!(prand([0xc0, 0x00, 0x00]), None);
    assert_eq!(prand([0x3f, 0xff, 0xff]), None);
    assert_eq!(prand([0xff, 0xff, 0xff]), None);
}

#[test]
fn another_key_or_another_kind_does_not_resolve() {
    let a = rpa(&irk(), &arr("708194"));
    let mut other = irk();
    other[15] ^= 1;
    assert!(!resolves(&other, &a));
    let mut flipped = a;
    flipped.0[0] ^= 1;
    assert!(!resolves(&irk(), &flipped));
    // A hash that does match, under a prefix that is not 0b01, is still
    // not a resolvable address (Vol 6, Part B, 1.3.2.3).
    for top in [0xc0, 0x00, 0x80] {
        let p = [0x30 | top, 0x81, 0x94];
        let h = ah(&irk(), &p);
        let kind = BdAddr([h[2], h[1], h[0], p[2], p[1], p[0]]);
        assert!(!resolves(&irk(), &kind), "{kind:?}");
    }
}
