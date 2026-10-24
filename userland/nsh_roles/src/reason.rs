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

//! The reason code each refusal is told as (nsh-v1-wire-session.md). A
//! failure in the proof of the key is always `USER_SIG`; an escalation's
//! failed issuer check is `GRANT_SIG`, since the user is already known.

use crate::Refusal;
use nsh_wire::Reason;

impl Refusal {
    /// The one code the peer is told.
    pub const fn reason(self) -> Reason {
        use Refusal::*;
        match self {
            Kind => Reason::Protocol,
            Issuer | IssuerSig | Subject | UserSig => Reason::UserSig,
            Fleet => Reason::GrantScope,
            NotYet | Expired | BaseEnded => Reason::GrantTime,
            RevocationStale => Reason::RevocationStale,
            Revoked => Reason::GrantRevoked,
            RoleUnknown => Reason::RoleUnknown,
            CapUnknown => Reason::CapUnknown,
            CapsCeiling => Reason::CapsCeiling,
            GrantSig => Reason::GrantSig,
            Parent => Reason::EscParent,
            TooLong => Reason::EscTooLong,
            ApprovalCount | ApprovalDuplicate | ApprovalOrder | ApprovalIssuer => {
                Reason::EscApproval
            }
            ApprovalSubject | ApprovalUnknown | ApprovalSig => Reason::EscApproval,
            Active => Reason::EscActive,
        }
    }
}
