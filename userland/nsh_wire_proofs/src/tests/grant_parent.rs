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

//! A base grant names no parent; an escalation names one, never itself,
//! and lists two approvers in ascending order.

use super::grant::parse;
use crate::build::grant::GrantB;
use nsh_wire::WireError;

#[test]
fn a_base_grant_has_no_parent_and_an_escalation_has_one_not_itself() {
    let d = GrantB::default;
    assert_eq!(parse(&GrantB { parent_id: [1; 16], ..d() }), Err(WireError::Value));
    let e = GrantB::escalation;
    assert_eq!(parse(&e()), Ok(()));
    assert_eq!(parse(&GrantB { parent_id: [0; 16], ..e() }), Err(WireError::Value));
    assert_eq!(parse(&GrantB { parent_id: [0x22; 16], ..e() }), Err(WireError::Value));
}

#[test]
fn two_approvals_come_in_ascending_order() {
    let e = GrantB::escalation;
    let down = GrantB { approvers: vec![[0xA2; 32], [0xA1; 32]], ..e() };
    assert_eq!(parse(&down), Err(WireError::Value));
    let mut lower = [0xA1; 32];
    lower[31] = 0xA0;
    assert_eq!(parse(&GrantB { approvers: vec![lower, [0xA1; 32]], ..e() }), Ok(()));
}
