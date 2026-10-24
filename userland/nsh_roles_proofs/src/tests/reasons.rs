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

//! Every refusal is told as the one code nsh-v1-wire-session.md gives it.

use nsh_roles::Refusal::*;
use nsh_wire::Reason;

#[test]
fn each_refusal_has_its_code() {
    let table = [
        (Kind, Reason::Protocol),
        (Issuer, Reason::UserSig),
        (IssuerSig, Reason::UserSig),
        (Subject, Reason::UserSig),
        (UserSig, Reason::UserSig),
        (Fleet, Reason::GrantScope),
        (NotYet, Reason::GrantTime),
        (Expired, Reason::GrantTime),
        (BaseEnded, Reason::GrantTime),
        (RevocationStale, Reason::RevocationStale),
        (Revoked, Reason::GrantRevoked),
        (RoleUnknown, Reason::RoleUnknown),
        (CapUnknown, Reason::CapUnknown),
        (CapsCeiling, Reason::CapsCeiling),
        (GrantSig, Reason::GrantSig),
        (Parent, Reason::EscParent),
        (TooLong, Reason::EscTooLong),
        (ApprovalCount, Reason::EscApproval),
        (ApprovalDuplicate, Reason::EscApproval),
        (ApprovalOrder, Reason::EscApproval),
        (ApprovalIssuer, Reason::EscApproval),
        (ApprovalSubject, Reason::EscApproval),
        (ApprovalUnknown, Reason::EscApproval),
        (ApprovalSig, Reason::EscApproval),
        (Active, Reason::EscActive),
    ];
    for (refusal, reason) in table {
        assert_eq!(refusal.reason(), reason, "{refusal:?}");
    }
}
