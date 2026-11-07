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

//! The reason codes a host gives when it refuses a login or ends a session
//! (nsh_wire::Reason), as a sentence.

use nsh_wire::Reason;

pub(super) fn reason(r: Reason) -> &'static str {
    match r {
        Reason::Ok => "it gave no reason (code 0)",
        Reason::Protocol => "it read a frame from this client that breaks nsh/1",
        Reason::Version => "it speaks another nsh version or suite",
        Reason::Policy => "its policy refused it",
        Reason::Limit => "a limit of the session was passed",
        Reason::Busy => "it is serving all the sessions it takes; try again later",
        Reason::Unattested => {
            "it has no TPM to attest with, so it serves no one; boot it with a \
                               TPM (a vTPM under QEMU)"
        }
        Reason::Unenrolled => {
            "it holds no enrolment certificate or role table yet; enrol the \
                               machine with nsh-enrol and put its records under /nonos/nsh"
        }
        Reason::GrantSig => "the grant's signature does not verify against its role table",
        Reason::GrantTime => "the grant is not valid at its clock: expired or not yet started",
        Reason::GrantScope => "the grant is not for this host or fleet",
        Reason::GrantRevoked => "the grant is revoked",
        Reason::RevocationStale => {
            "its own revocation list is out of date; it admits no one \
                                    until it is given a fresh one"
        }
        Reason::RoleUnknown => "the grant names a role its role table does not hold",
        Reason::CapsCeiling => "the grant asks for more than its role allows",
        Reason::CapUnknown => "the grant names a capability it does not know",
        Reason::UserSig => "the user key's signature does not verify",
        Reason::EscParent => {
            "the escalation grant is not for this session's base grant, user, role or fleet"
        }
        Reason::EscApproval => "the escalation does not carry the approvals its role demands",
        Reason::EscTooLong => "the escalation asks for a window over one hour",
        Reason::EscActive => "an escalation is already active",
    }
}
