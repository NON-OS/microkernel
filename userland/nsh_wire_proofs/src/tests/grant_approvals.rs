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

//! The approvals an escalation carries: none on a base grant, at most two,
//! never the issuer, never the same approver twice.

use super::grant::parse;
use crate::build::grant::GrantB;
use nsh_wire::{Grant, GrantKind, WireError};

#[test]
fn an_escalation_carries_its_approvals_in_order() {
    let b = GrantB { approvers: vec![[0xA1; 32], [0xA2; 32]], ..GrantB::escalation() };
    let bytes = b.bytes();
    let g = Grant::parse(&bytes).expect("valid");
    assert_eq!(g.kind, GrantKind::Escalation);
    let ids: Vec<_> = g.approvals().map(|a| *a.approver).collect();
    assert_eq!(ids, [[0xA1; 32], [0xA2; 32]]);
    assert!(g.approvals().all(|a| a.sig.ed[0] == 0xA5 && a.sig.ml[3308] == 0xA5));
}

#[test]
fn an_escalation_may_carry_none() {
    assert_eq!(parse(&GrantB { approvers: vec![], ..GrantB::escalation() }), Ok(()));
}

#[test]
fn a_base_grant_carries_none() {
    let b = GrantB { approvers: vec![[0xA1; 32]], ..GrantB::default() };
    assert_eq!(parse(&b), Err(WireError::Value));
}

#[test]
fn three_approvals_are_refused() {
    let b = GrantB { approvers: vec![[1; 32], [2; 32], [3; 32]], ..GrantB::escalation() };
    assert_eq!(parse(&b), Err(WireError::Value));
}

#[test]
fn the_issuer_cannot_approve_and_no_one_approves_twice() {
    let e = GrantB::escalation;
    let issuer = e().issuer;
    assert_eq!(parse(&GrantB { approvers: vec![issuer], ..e() }), Err(WireError::Value));
    let twice = GrantB { approvers: vec![[0xA1; 32], [0xA1; 32]], ..e() };
    assert_eq!(parse(&twice), Err(WireError::Value));
    let second_is_issuer = GrantB { approvers: vec![[0xA1; 32], issuer], ..e() };
    assert_eq!(parse(&second_is_issuer), Err(WireError::Value));
}

#[test]
fn a_cut_approval_is_short_and_a_padded_grant_trailing() {
    let bytes = GrantB::escalation().bytes();
    assert_eq!(Grant::parse(&bytes[..bytes.len() - 1]), Err(WireError::Short));
    assert_eq!(Grant::parse(&[bytes, vec![0]].concat()), Err(WireError::Trailing));
}
