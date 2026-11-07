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

//! The answer to a connection the host cannot take: a plaintext
//! `ABORT(BUSY)`, the one refusal besides `VERSION` that can be told
//! before keys exist. A plaintext frame's length is its type byte and
//! body; a two-byte code is always within the hello's cap.

use alloc::vec::Vec;
use nsh_wire::{Kind, Reason};

/// The whole frame to send before closing a connection the host is too
/// busy to serve.
pub fn busy() -> Vec<u8> {
    plain_abort(Reason::Busy)
}

/// The whole frame that turns a connection away before keys for `reason`,
/// or `None` for a reason nsh/1 lets no host tell in plaintext: only
/// `VERSION`, `BUSY`, `UNATTESTED` and `UNENROLLED` may be.
pub fn turned_away(reason: Reason) -> Option<Vec<u8>> {
    reason.before_keys().then(|| plain_abort(reason))
}

pub(crate) fn plain_abort(reason: Reason) -> Vec<u8> {
    let body = reason.code().to_le_bytes();
    let mut out = Vec::with_capacity(4 + 1 + body.len());
    out.extend_from_slice(&3u32.to_le_bytes());
    out.push(Kind::Abort.byte());
    out.extend_from_slice(&body);
    out
}
