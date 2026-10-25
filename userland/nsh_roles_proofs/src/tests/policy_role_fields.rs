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

//! A role's flags, approval count and longest session are each held to
//! their range.

use super::policy_roles::with;
use crate::build::table::TableB;
use nsh_roles::{open_policy, PolicyError, MAX_SPAN_MS};
use nsh_wire::WireError;

#[test]
fn only_the_digest_flag_is_defined() {
    assert_eq!(with(|r| r.flags = 2), Err(WireError::Reserved.into()));
    assert_eq!(with(|r| r.flags = 0x80), Err(WireError::Reserved.into()));
    let mut t = TableB::default();
    t.role("operator").flags = 1;
    let bytes = t.bytes();
    let p = open_policy(&bytes, &t.anchor, 0).expect("opens");
    assert!(p.role(b"operator").expect("role").record_digest);
    assert!(!p.role(b"auditor").expect("role").record_digest);
}

#[test]
fn an_escalation_needs_zero_one_or_two_approvals() {
    (0..=2).for_each(|n| assert_eq!(with(|r| r.approvals = n), Ok(())));
    assert_eq!(with(|r| r.approvals = 3), Err(PolicyError::Range));
}

#[test]
fn the_longest_session_is_from_one_ms_to_seven_days() {
    assert_eq!(with(|r| r.max_session_ms = 0), Err(PolicyError::Range));
    assert_eq!(with(|r| r.max_session_ms = 1), Ok(()));
    assert_eq!(with(|r| r.max_session_ms = MAX_SPAN_MS), Ok(()));
    assert_eq!(with(|r| r.max_session_ms = MAX_SPAN_MS + 1), Err(PolicyError::Range));
}
