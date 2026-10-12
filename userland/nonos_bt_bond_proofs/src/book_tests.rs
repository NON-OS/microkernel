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

//! Finding a bonded peer: by its public or static identity, or by the IRK
//! that resolves the private address it advertises from.

use crate::fixture::{public, with_irk, IRK};
use nonos_bt_bond::Book;
use nonos_bt_smp::privacy::rpa;
use nonos_bt_wire::addr::{AddrType, BdAddr};

pub fn book() -> Book {
    let mut b = Book::default();
    b.insert(with_irk()).unwrap();
    b.insert(public()).unwrap();
    b
}

#[test]
fn identities_are_found_by_type_and_address() {
    let b = book();
    let found = |k, a: [u8; 6]| b.find(k, &BdAddr(a)).map(|r| r.ltk);
    assert_eq!(found(AddrType::Public, [1, 2, 3, 4, 5, 6]), Some(public().ltk));
    assert_eq!(found(AddrType::Random, with_irk().identity), Some(with_irk().ltk));
    // The same octets under the other type are another device.
    assert_eq!(found(AddrType::Public, with_irk().identity), None);
    assert_eq!(found(AddrType::Random, [1, 2, 3, 4, 5, 6]), None);
}

#[test]
fn a_private_address_is_found_by_the_irk_that_resolves_it() {
    let b = book();
    let addr = rpa(&IRK, &[0x52, 0x34, 0x56]);
    assert_eq!(b.find(AddrType::Random, &addr).map(|r| r.identity), Some(with_irk().identity));
    let stranger = rpa(b"some-other-irk!!", &[0x52, 0x34, 0x56]);
    assert!(b.find(AddrType::Random, &stranger).is_none());
    let non_resolvable = BdAddr([addr.0[0], addr.0[1], addr.0[2], 1, 2, 0x12]);
    assert!(b.find(AddrType::Random, &non_resolvable).is_none());
}
