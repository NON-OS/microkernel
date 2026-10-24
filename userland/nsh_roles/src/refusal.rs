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

//! Why a login or an escalation was refused. Each check has its own
//! refusal, so the host's refusal log says exactly which check failed; the
//! peer is told only the reason code, and every failure in the first part
//! of a login, the proof of the key, is told the one code `USER_SIG`.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refusal {
    /// A login's grant is an escalation, or an escalation's is a base.
    Kind,
    /// The issuer is not a role authority in the table.
    Issuer,
    /// The issuer's signatures do not both verify.
    IssuerSig,
    /// The subject is not H(the keys in `USER_AUTH`).
    Subject,
    /// The user's signatures over the transcript do not both verify.
    UserSig,
    /// The grant is for another fleet.
    Fleet,
    NotYet,
    Expired,
    RevocationStale,
    Revoked,
    RoleUnknown,
    CapUnknown,
    CapsCeiling,
    /// An escalation's issuer is unknown or its signatures do not verify.
    GrantSig,
    /// The parent is not the base grant, or the user, subject, role or
    /// fleet differ from the base grant's.
    Parent,
    /// The base session has already ended.
    BaseEnded,
    TooLong,
    /// Not the number of approvals the role demands.
    ApprovalCount,
    ApprovalDuplicate,
    ApprovalOrder,
    ApprovalIssuer,
    ApprovalSubject,
    /// The approver key is not one the table lists as an approver.
    ApprovalUnknown,
    ApprovalSig,
    Active,
}
