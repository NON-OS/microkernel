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

//! Which reasons each frame may carry. A code outside its frame's set is
//! refused, so a peer cannot end a session "because the grant expired" or
//! refuse a login "because of the version".

use crate::Reason;

impl Reason {
    /// An `ABORT` sent in plaintext, before any key exists: only these can
    /// be told then, since nothing about the peer is known yet.
    pub const fn before_keys(self) -> bool {
        matches!(self, Reason::Version | Reason::Busy)
    }

    /// A sealed `ABORT`: the protocol broke, a limit was met, or the client
    /// refused the host.
    pub const fn ends_session(self) -> bool {
        matches!(self.code(), 1..=5)
    }

    /// `AUTH_REFUSED`: why the login was refused.
    pub const fn refuses_login(self) -> bool {
        matches!(self.code(), 16..=24)
    }

    /// `ESCALATE_RESULT` and the record's `ESC_REFUSED`: why an escalation
    /// was refused, which may be anything that would refuse a grant.
    pub const fn refuses_escalation(self) -> bool {
        matches!(self.code(), 16..=24 | 32..=35)
    }
}
