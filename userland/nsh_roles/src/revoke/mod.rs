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

//! The signed revocation list, `nsh-revoke/1`. It names revoked grants by
//! `grant_id`, never by hash, so one grant cannot be given a second hash
//! that escapes the list. It is signed by a role authority of the table it
//! is opened against, for that table's fleet, and carries its issue time:
//! a host whose list is too old admits nobody.

mod parse;
mod verify;

use nsh_wire::GRANT_ID_LEN;

pub use parse::RevocationList;
pub use verify::open_revocations;

pub const REVOKE_MAGIC: [u8; 4] = *b"NSHV";
pub const REVOKE_VERSION: u16 = 1;
/// The most grant ids one list names.
pub const MAX_REVOKED: usize = 4096;

/// A list whose signature verified under one of the table's authorities.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Revocations<'a>(pub(crate) RevocationList<'a>);

impl<'a> Revocations<'a> {
    pub const fn list(&self) -> &RevocationList<'a> {
        &self.0
    }

    /// Issued at or before `now`, and at most `max_age_ms` before it. A list
    /// from the future means the host clock is wrong, and is not fresh.
    pub fn fresh(&self, now_ms: u64, max_age_ms: u64) -> bool {
        now_ms.checked_sub(self.0.issued_ms).is_some_and(|age| age <= max_age_ms)
    }

    pub fn contains(&self, id: &[u8; GRANT_ID_LEN]) -> bool {
        self.0.ids().any(|r| r == id.as_slice())
    }
}

impl<'a> RevocationList<'a> {
    pub fn len(&self) -> usize {
        self.ids.len() / GRANT_ID_LEN
    }

    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }

    pub fn ids(&self) -> impl Iterator<Item = &'a [u8]> + 'a {
        self.ids.chunks_exact(GRANT_ID_LEN)
    }
}
