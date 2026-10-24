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

//! Each approval is signed by the approver it names, over the approval
//! digest and not the grant digest, with both halves verifying.

use super::escalate_approvals::{by, with};
use crate::build::grant::ApprovalB;
use crate::build::keys::{key, K};
use crate::build::Spoil;
use nsh_roles::Refusal;

#[test]
fn each_approval_is_signed_by_its_approver_over_the_approval_digest() {
    let wrong = ApprovalB { signer: K::Appr2, ..by(K::Appr1) };
    assert_eq!(with(1, vec![wrong.clone()]), Err(Refusal::ApprovalSig));
    assert_eq!(with(2, vec![by(K::Appr2), wrong]), Err(Refusal::ApprovalSig));
    let unlabelled = ApprovalB { unlabelled: true, ..by(K::Appr1) };
    assert_eq!(with(1, vec![unlabelled]), Err(Refusal::ApprovalSig));
    for spoil in [Spoil::Ed, Spoil::Ml] {
        assert_eq!(with(1, vec![ApprovalB { spoil, ..by(K::Appr1) }]), Err(Refusal::ApprovalSig));
    }
    assert_eq!(key(K::Appr1).id(), by(K::Appr1).id);
}
