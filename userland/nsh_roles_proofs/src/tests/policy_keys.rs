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

//! The table's key lists: one to eight authorities, up to sixteen
//! approvers, each list strictly ascending by key id, and no key in both.

use super::policy::{open, parse};
use crate::build::keys::K;
use crate::build::table::TableB;
use nsh_roles::PolicyError;

/// The authority count sits after the preamble, the fleet, the serial, the
/// issue time and the age.
pub const AUTH_COUNT_AT: usize = 8 + 16 + 8 + 8 + 8;
pub const KEY_LEN: usize = 32 + 1952;

#[test]
fn a_table_needs_one_to_eight_authorities() {
    let none = TableB { authorities: vec![], ..TableB::default() };
    assert_eq!(open(&none), Err(PolicyError::Range));
    let one = TableB { authorities: vec![K::AuthA], ..TableB::default() };
    assert_eq!(open(&one), Ok(()));
    let mut b = TableB::default().bytes();
    b[AUTH_COUNT_AT] = 9;
    assert_eq!(parse(&b), Err(PolicyError::Range));
}

/// Two keys whose ids are in descending order as listed.
pub fn descending(a: K, b: K) -> Vec<K> {
    let (ia, ib) = (crate::build::keys::key(a).id(), crate::build::keys::key(b).id());
    if ia > ib {
        vec![a, b]
    } else {
        vec![b, a]
    }
}

#[test]
fn authorities_out_of_order_or_twice_are_refused() {
    let base = || TableB { sort_keys: false, approvers: vec![K::Appr1], ..TableB::default() };
    let t = TableB { authorities: descending(K::AuthA, K::AuthB), ..base() };
    assert_eq!(open(&t), Err(PolicyError::Order));
    let t = TableB { authorities: vec![K::AuthA, K::AuthA], ..base() };
    assert_eq!(open(&t), Err(PolicyError::Order));
}
