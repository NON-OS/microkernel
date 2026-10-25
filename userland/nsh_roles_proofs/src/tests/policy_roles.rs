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

//! Each role in a table: its count, order, name, flags, approvals and
//! longest session are each held to their rule.

use super::policy::{open, parse};
use crate::build::table::{RoleB, TableB};
use nsh_roles::PolicyError;
use nsh_wire::WireError;

pub fn with(f: impl FnOnce(&mut RoleB)) -> Result<(), PolicyError> {
    let mut t = TableB::default();
    f(t.role("operator"));
    open(&t)
}

#[test]
fn a_table_has_one_to_sixteen_roles() {
    let mut t = TableB::default();
    t.roles.truncate(1);
    assert_eq!(open(&t), Ok(()));
    t.roles.clear();
    assert_eq!(open(&t), Err(PolicyError::Range));
    let t = TableB::default();
    let at = t.signed().len() - 32 - roles_len(&t);
    let mut b = t.bytes();
    assert_eq!(b[at], 3);
    b[at] = 17;
    assert_eq!(parse(&b), Err(PolicyError::Range));
}

/// The bytes the roles take, their count included.
fn roles_len(t: &TableB) -> usize {
    let one = TableB { roles: vec![], ..t.clone() }.signed().len();
    t.signed().len() - one + 1
}

#[test]
fn roles_out_of_order_or_twice_are_refused() {
    let mut t = TableB::default();
    t.roles.swap(0, 1);
    assert_eq!(open(&t), Err(PolicyError::Order));
    let mut t = TableB::default();
    t.roles[1] = t.roles[0].clone();
    assert_eq!(open(&t), Err(PolicyError::Order));
}

#[test]
fn a_role_name_keeps_its_shape() {
    assert_eq!(with(|r| r.name = b"Operator".to_vec()), Err(WireError::Value.into()));
    assert_eq!(with(|r| r.name = b"op_erator".to_vec()), Err(WireError::Value.into()));
    assert_eq!(with(|r| r.name = vec![b'p'; 17]), Err(WireError::Oversize.into()));
    assert_eq!(with(|r| r.name = vec![b'p'; 16]), Ok(()));
}
