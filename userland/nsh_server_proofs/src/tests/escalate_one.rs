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

//! `ESCALATE`: the role table checks it against the session's own login,
//! one at a time; a refusal is answered with its code and recorded, and
//! the session goes on; a granted escalation runs until its window closes,
//! its grant is revoked, or the session ends, and the record says which.

use super::escalate::*;
use crate::kit::roles_build::grant::{ApprovalB, GrantB};
use crate::kit::roles_build::keys::K;
use crate::kit::roles_build::{HOUR, NOW};
use crate::kit::run::Run;
use nsh_roles::Refusal as R;
use nsh_server::Why;
use nsh_wire::Reason;

#[test]
fn each_escalation_refusal_is_answered_and_recorded() {
    refused_with(
        GrantB { approvals: vec![], ..GrantB::escalation() },
        Reason::EscApproval,
        R::ApprovalCount,
    );
    let unknown = ApprovalB::by(K::Stranger);
    refused_with(
        GrantB { approvals: vec![unknown], ..GrantB::escalation() },
        Reason::EscApproval,
        R::ApprovalUnknown,
    );
    refused_with(
        GrantB { parent_id: [0x33; 16], ..GrantB::escalation() },
        Reason::EscParent,
        R::Parent,
    );
    let long =
        GrantB { not_before_ms: NOW - 60_000, not_after_ms: NOW + HOUR, ..GrantB::escalation() };
    refused_with(long, Reason::EscTooLong, R::TooLong);
    let wide = GrantB { caps: vec!["CoreExec", "Debug"], ..GrantB::escalation() };
    refused_with(wide, Reason::CapsCeiling, R::CapsCeiling);
}

#[test]
fn one_escalation_at_a_time() {
    let mut run = Run::logged_in();
    run.user(|c| c.escalate(&GrantB::escalation().bytes())).expect("escalate");
    let again = GrantB { grant_id: [0x23; 16], ..GrantB::escalation() };
    run.user(|c| c.escalate(&again.bytes())).expect("escalate again");
    assert_eq!(escalated(&run)[1], (Reason::EscActive, 0));
    assert_eq!(run.refused(), [Why::Escalation(R::Active)]);
}
