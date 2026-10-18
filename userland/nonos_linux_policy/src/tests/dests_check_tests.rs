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

//! A list is accepted in exactly one spelling.

use super::dests_write::{entry, list, name, raw, v4};
use crate::dests::{check, DestsError as E};

fn one(e: Vec<u8>) -> Result<usize, E> {
    check(&raw(&[e]))
}

#[test]
fn a_canonical_list_is_counted() {
    let l = list(vec![v4(24, (443, 443), [10, 1, 2, 0]), name("example.org", (1, 65535))]);
    assert_eq!(check(&l), Ok(2));
    assert_eq!(one(entry(6, 0, (80, 80), &[0; 16])), Ok(1));
}

#[test]
fn the_count_bounds_the_list() {
    assert_eq!(check(&[]), Err(E::Short));
    assert_eq!(check(&[0]), Err(E::Short));
    assert_eq!(check(&[0, 0]), Err(E::Count));
    let many: Vec<_> = (0..65u8).map(|i| v4(32, (1, 1), [10, 0, 0, i])).collect();
    assert_eq!(check(&list(many.clone())), Err(E::Count));
    assert_eq!(check(&list(many[..64].to_vec())), Ok(64));
}

#[test]
fn kinds_ports_and_addresses_are_checked() {
    assert_eq!(one(entry(5, 0, (1, 1), &[0; 4])), Err(E::Kind));
    assert_eq!(one(v4(0, (0, 80), [0; 4])), Err(E::Ports));
    assert_eq!(one(v4(0, (81, 80), [0; 4])), Err(E::Ports));
    assert_eq!(one(entry(4, 0, (1, 1), &[0; 3])), Err(E::Address));
    assert_eq!(one(v4(33, (1, 1), [0; 4])), Err(E::Address));
    assert_eq!(one(v4(24, (1, 1), [10, 1, 2, 1])), Err(E::HostBits));
    assert_eq!(one(v4(31, (1, 1), [10, 1, 2, 1])), Err(E::HostBits));
    assert_eq!(one(v4(32, (1, 1), [10, 1, 2, 1])), Ok(1));
    let mapped = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff, 1, 2, 3, 4];
    assert_eq!(one(entry(6, 128, (1, 1), &mapped)), Err(E::Address));
    let mut block = mapped;
    block[12..].fill(0);
    assert_eq!(one(entry(6, 96, (1, 1), &block)), Err(E::Address));
    assert_eq!(one(entry(6, 80, (1, 1), &[0; 16])), Ok(1));
    assert_eq!(one(entry(6, 129, (1, 1), &[0; 16])), Err(E::Address));
}

#[test]
fn entries_come_in_order_once_and_nothing_follows() {
    let (a, b) = (v4(32, (1, 1), [10, 0, 0, 1]), v4(32, (1, 1), [10, 0, 0, 2]));
    assert_eq!(check(&raw(&[b.clone(), a.clone()])), Err(E::Order));
    assert_eq!(check(&raw(&[a.clone(), a.clone()])), Err(E::Order));
    let mut l = raw(&[a.clone(), b]);
    l.push(0);
    assert_eq!(check(&l), Err(E::Trailing));
    assert_eq!(check(&raw(&[a])[..8]), Err(E::Short));
}
