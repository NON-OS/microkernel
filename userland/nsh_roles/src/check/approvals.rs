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

//! An escalation's approvals. There are exactly as many as the role
//! demands, in strictly ascending approver order, each from an approver key
//! the table lists, none from the subject or the issuer, and each signed
//! over `H("nsh/1 approve\0" || H("nsh/1 grant\0" || signed bytes))`.

use crate::check::Host;
use crate::hybrid::verify;
use crate::{Policy, Refusal};
use alloc::vec::Vec;
use nsh_crypto::{labelled, Label};
use nsh_wire::keys::{DIGEST_LEN, KEY_ID_LEN};
use nsh_wire::{Approval, Grant};

/// The longest window an escalation grant may name: one hour.
pub const MAX_ESCALATION_MS: u64 = 3_600_000;

pub(crate) fn check_approvals(host: &Host<'_>, g: &Grant<'_>, need: u8) -> Result<(), Refusal> {
    let digest = labelled(Label::Approve, &labelled(Label::Grant, g.signed));
    let list: Vec<Approval<'_>> = g.approvals().collect();
    approvals(host.policy, g.subject, g.issuer, &digest, &list, need)
}

/// The approval rules on a list as given. The grant reader already refuses
/// a repeated, out-of-order or issuer approval; this holds them again for
/// any list, however it was read.
pub fn approvals(
    policy: &Policy<'_>,
    subject: &[u8; DIGEST_LEN],
    issuer: &[u8; KEY_ID_LEN],
    digest: &[u8; DIGEST_LEN],
    list: &[Approval<'_>],
    need: u8,
) -> Result<(), Refusal> {
    if list.len() != usize::from(need) {
        return Err(Refusal::ApprovalCount);
    }
    let mut before: Option<&[u8; KEY_ID_LEN]> = None;
    for a in list {
        if before == Some(a.approver) {
            return Err(Refusal::ApprovalDuplicate);
        }
        if before.is_some_and(|b| a.approver < b) {
            return Err(Refusal::ApprovalOrder);
        }
        if a.approver == issuer {
            return Err(Refusal::ApprovalIssuer);
        }
        let key = policy.approver(a.approver).ok_or(Refusal::ApprovalUnknown)?;
        if &key.subject == subject {
            return Err(Refusal::ApprovalSubject);
        }
        if !verify(&key.public, digest, &a.sig) {
            return Err(Refusal::ApprovalSig);
        }
        before = Some(a.approver);
    }
    Ok(())
}
