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

//! An escalation's parent is the session's base grant, and it names the
//! base grant's user, subject, role and fleet. So an escalation of an
//! escalation is refused, and so is one that names a role for itself.

use super::escalate::esc;
use crate::build::grant::GrantB;
use crate::build::keys::{key, K};
use crate::build::run::Setup;
use crate::build::ESC_ID;
use nsh_roles::Refusal;
use nsh_wire::Reason;

#[test]
fn the_parent_must_be_the_base_grant() {
    let g = GrantB { parent_id: [0x12; 16], ..GrantB::escalation() };
    assert_eq!(esc(g), Err(Refusal::Parent));
    assert_eq!(Refusal::Parent.reason(), Reason::EscParent);
}

#[test]
fn an_escalation_of_an_escalation_is_refused() {
    let second = GrantB { grant_id: [0x33; 16], parent_id: ESC_ID, ..GrantB::escalation() };
    assert_eq!(esc(second), Err(Refusal::Parent));
}

#[test]
fn the_user_must_be_the_base_grants() {
    assert_eq!(esc(GrantB { user: b"ek2".to_vec(), ..GrantB::escalation() }), Err(Refusal::Parent));
}

#[test]
fn the_subject_must_be_the_base_grants() {
    let g = GrantB { subject: key(K::OtherUser).subject(), ..GrantB::escalation() };
    assert_eq!(esc(g), Err(Refusal::Parent));
}

#[test]
fn the_role_is_the_base_grants_never_one_the_escalation_names() {
    let g = GrantB { role: b"deploy".to_vec(), ..GrantB::escalation() };
    assert_eq!(esc(g), Err(Refusal::Parent));
}

#[test]
fn the_fleet_must_be_the_base_grants() {
    assert_eq!(esc(GrantB { fleet: [0xF2; 16], ..GrantB::escalation() }), Err(Refusal::Parent));
}

#[test]
fn the_parent_check_comes_before_the_time() {
    let s = Setup::default();
    let g = GrantB {
        parent_id: [0x12; 16],
        not_after_ms: 1 + 1,
        not_before_ms: 1,
        ..GrantB::escalation()
    };
    assert_eq!(s.escalate(&Setup::base(), &g).err(), Some(Refusal::Parent));
}
