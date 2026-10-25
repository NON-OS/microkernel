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

//! What a record must name to be the one the caller expects: the session
//! id `AUTH_OK` gave, the handshake's `TH(SERVER_HELLO)` and the user's
//! subject. A host holding a valid record of another session or another
//! user cannot pass it off as this one's.

use crate::ct::eq;
use nsh_wire::RecordHeader;

/// The fields a record must hold; `None` leaves that field unchecked.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Bound {
    pub session_id: Option<[u8; 16]>,
    pub transcript: Option<[u8; 64]>,
    pub subject: Option<[u8; 64]>,
}

/// The field a record names wrongly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mismatch {
    SessionId,
    Transcript,
    Subject,
    /// The client's own check: the document hash is not that of the
    /// document `HOST_PROOF` carried.
    Document,
}

pub(crate) fn check(h: &RecordHeader<'_>, b: &Bound) -> Result<(), Mismatch> {
    let differs = |want: Option<&[u8]>, got: &[u8]| want.is_some_and(|w| !eq(w, got));
    if differs(b.session_id.as_ref().map(|s| &s[..]), h.session_id) {
        return Err(Mismatch::SessionId);
    }
    if differs(b.transcript.as_ref().map(|t| &t[..]), h.transcript) {
        return Err(Mismatch::Transcript);
    }
    match differs(b.subject.as_ref().map(|s| &s[..]), h.subject) {
        true => Err(Mismatch::Subject),
        false => Ok(()),
    }
}
