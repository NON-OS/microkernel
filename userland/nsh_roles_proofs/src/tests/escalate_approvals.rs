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

//! An escalation carries exactly the approvals its role demands, each from
//! an approver key the table lists, never the subject's, each signed over
//! the approval digest with both halves verifying.

use crate::build::grant::{ApprovalB, GrantB};
use crate::build::keys::K;
use crate::build::run::Setup;
use nsh_roles::Refusal;
use nsh_wire::Reason;

pub fn with(need: u8, approvals: Vec<ApprovalB>) -> Result<(), Refusal> {
    let mut s = Setup::default();
    s.table.role("operator").approvals = need;
    let base = s.login(&GrantB::default()).expect("base");
    s.escalate(&base, &GrantB { approvals, ..GrantB::escalation() }).map(|_| ())
}

pub fn by(k: K) -> ApprovalB {
    ApprovalB::by(k)
}

#[test]
fn exactly_the_number_the_role_demands() {
    assert_eq!(with(0, vec![]), Ok(()));
    assert_eq!(with(0, vec![by(K::Appr1)]), Err(Refusal::ApprovalCount));
    assert_eq!(with(1, vec![]), Err(Refusal::ApprovalCount));
    assert_eq!(with(1, vec![by(K::Appr1), by(K::Appr2)]), Err(Refusal::ApprovalCount));
    assert_eq!(with(2, vec![by(K::Appr1)]), Err(Refusal::ApprovalCount));
    assert_eq!(with(2, vec![by(K::Appr2), by(K::Appr3)]), Ok(()));
    assert_eq!(Refusal::ApprovalCount.reason(), Reason::EscApproval);
}

#[test]
fn an_approver_must_be_listed_as_one() {
    assert_eq!(with(1, vec![by(K::Stranger)]), Err(Refusal::ApprovalUnknown));
    assert_eq!(with(1, vec![by(K::AuthB)]), Err(Refusal::ApprovalUnknown));
    assert_eq!(with(2, vec![by(K::Appr1), by(K::OtherUser)]), Err(Refusal::ApprovalUnknown));
}

#[test]
fn an_approver_is_never_the_subject() {
    let mut s = Setup::default();
    s.table.approvers.push(K::User);
    let base = s.login(&GrantB::default()).expect("base");
    let g = GrantB { approvals: vec![by(K::User)], ..GrantB::escalation() };
    assert_eq!(s.escalate(&base, &g).err(), Some(Refusal::ApprovalSubject));
    let g = GrantB { approvals: vec![by(K::Appr1)], ..GrantB::escalation() };
    assert!(s.escalate(&base, &g).is_ok());
}
