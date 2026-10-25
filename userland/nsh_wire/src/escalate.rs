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

//! `ESCALATE`: one grant, which must be an escalation grant. Whether it
//! belongs to this session's base grant is the server's check.

use crate::user_auth::MAX_GRANT;
use crate::{Grant, Reader, WireError};

/// An `ESCALATE` body: one grant, which must be an escalation grant.
pub fn escalate(body: &[u8]) -> Result<Grant<'_>, WireError> {
    let mut r = Reader::new(body);
    let grant = Grant::parse(r.bytes16(1, MAX_GRANT)?)?;
    r.finish()?;
    match grant.kind {
        crate::GrantKind::Escalation => Ok(grant),
        crate::GrantKind::Base => Err(WireError::Value),
    }
}
