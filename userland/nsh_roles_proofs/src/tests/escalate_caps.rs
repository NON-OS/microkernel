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

//! An escalation's capabilities are known names within the base role's
//! escalation ceiling; a role whose escalation ceiling is empty never
//! escalates.

use super::escalate::esc;
use crate::build::grant::{ApprovalB, GrantB};
use crate::build::keys::K;
use crate::build::run::Setup;
use nsh_roles::caps::FORBIDDEN;
use nsh_roles::Refusal;

#[test]
fn capabilities_must_be_known_and_within_the_escalation_ceiling() {
    let caps = |c: Vec<&'static str>| esc(GrantB { caps: c, ..GrantB::escalation() });
    assert_eq!(caps(vec!["CoreExec", "Root"]), Err(Refusal::CapUnknown));
    assert_eq!(caps(vec!["CoreExec", "Driver"]), Err(Refusal::CapsCeiling));
    assert_eq!(caps(vec!["CoreExec", "TimeSet"]), Err(Refusal::CapsCeiling));
    for name in FORBIDDEN {
        assert_eq!(caps(vec!["Network", name]), Err(Refusal::CapsCeiling), "{name}");
    }
    assert_eq!(caps(vec!["Network", "StoreWrite"]), Ok(()));
}

#[test]
fn a_role_with_an_empty_escalation_ceiling_never_escalates() {
    let s = Setup::default();
    let base = GrantB { role: b"auditor".to_vec(), caps: vec!["CoreExec"], ..GrantB::default() };
    let base = s.login(&base).expect("auditor login");
    let approvals = vec![ApprovalB::by(K::Appr1), ApprovalB::by(K::Appr2)];
    let g = GrantB {
        role: b"auditor".to_vec(),
        caps: vec!["CoreExec"],
        approvals,
        ..GrantB::escalation()
    };
    assert_eq!(s.escalate(&base, &g).err(), Some(Refusal::CapsCeiling));
}
