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
//! One end for the whole TLS handshake to an RPC host (A9e's rule, here
//! for the wallet's own stepped client): from the hello sent to the server's
//! flight checked, however slowly the host trickles bytes in. Each gather's
//! own quiet and total bounds still hold inside it. Pure.

/// The handshake's whole span, as nonos_tls::HANDSHAKE_MS sets it (A9e);
/// that name replaces this one once it is on this branch.
pub const HANDSHAKE_MS: u64 = 30_000;

/// The end for a handshake whose hello goes at `now_ms`.
pub fn end_at(now_ms: u64) -> u64 {
    now_ms.saturating_add(HANDSHAKE_MS)
}

/// Whether a handshake that must end at `end` has run out at `now_ms`.
pub fn over(end: Option<u64>, now_ms: u64) -> bool {
    end.is_some_and(|e| now_ms >= e)
}
