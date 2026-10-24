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

//! The approver list: at most sixteen, strictly ascending by key id, and
//! no approver key is also a role authority.

use super::policy::{open, parse};
use super::policy_keys::{descending, AUTH_COUNT_AT, KEY_LEN};
use crate::build::keys::K;
use crate::build::table::TableB;
use nsh_roles::PolicyError;

#[test]
fn approvers_out_of_order_or_twice_are_refused() {
    let base = || TableB { sort_keys: false, authorities: vec![K::AuthA], ..TableB::default() };
    let t = TableB { approvers: descending(K::Appr1, K::Appr2), ..base() };
    assert_eq!(open(&t), Err(PolicyError::Order));
    let t = TableB { approvers: vec![K::Appr2, K::Appr2], ..base() };
    assert_eq!(open(&t), Err(PolicyError::Order));
}

#[test]
fn no_key_is_both_an_authority_and_an_approver() {
    let t = TableB { approvers: vec![K::Appr1, K::AuthB], ..TableB::default() };
    assert_eq!(open(&t), Err(PolicyError::Overlap));
    let t = TableB { approvers: vec![K::AuthA], ..TableB::default() };
    assert_eq!(open(&t), Err(PolicyError::Overlap));
}

#[test]
fn a_table_has_at_most_sixteen_approvers() {
    let none = TableB { approvers: vec![], ..TableB::default() };
    assert_eq!(open(&none), Ok(()));
    let mut b = TableB::default().bytes();
    b[AUTH_COUNT_AT + 1 + 2 * KEY_LEN] = 17;
    assert_eq!(parse(&b), Err(PolicyError::Range));
}
