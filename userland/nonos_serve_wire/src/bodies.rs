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

//! The bodies READ and WRITE carry. Each opens with the asker's number for
//! the exchange, u32 LE. The kernel drops an answer whose caller stopped
//! waiting, and READ's answer takes bytes off the guest's buffer, so the
//! personality keeps each connection's last answer: a question asked again
//! under the same number gets that answer again, nothing taken twice and
//! nothing lost, as the proxies' kept answers do (capsule_net_anon
//! server/socks/kept.rs). The next number starts a new exchange.

use crate::ops::{MAX_BODY, MAX_WAIT_MS};

/// READ's question: the number, how much the asker takes, how long it waits.
pub fn read_ask(seq: u32, want: u32, wait_ms: u32) -> [u8; 12] {
    let mut out = [0u8; 12];
    out[..4].copy_from_slice(&seq.to_le_bytes());
    out[4..8].copy_from_slice(&want.to_le_bytes());
    out[8..].copy_from_slice(&wait_ms.to_le_bytes());
    out
}

/// A READ question read back as (number, want, wait_ms), refused past the bounds.
pub fn read_asked(body: &[u8]) -> Option<(u32, u32, u32)> {
    let word = |at: usize| Some(u32::from_le_bytes(body.get(at..at + 4)?.try_into().ok()?));
    let (seq, want, wait) = (word(0)?, word(4)?, word(8)?);
    let fits = body.len() == 12 && want > 0 && want as usize <= MAX_BODY;
    (fits && wait <= MAX_WAIT_MS).then_some((seq, want, wait))
}

/// WRITE's question opens with its number; the bytes for the guest follow.
pub fn write_head(seq: u32) -> [u8; 4] {
    seq.to_le_bytes()
}

/// A WRITE question read back as (number, bytes), at least one byte.
pub fn written(body: &[u8]) -> Option<(u32, &[u8])> {
    let seq = u32::from_le_bytes(body.get(..4)?.try_into().ok()?);
    let bytes = &body[4..];
    (!bytes.is_empty()).then_some((seq, bytes))
}

/// WRITE's answer body: how many bytes the guest's buffer took.
pub fn took(n: u32) -> [u8; 4] {
    n.to_le_bytes()
}

/// A WRITE answer's body read back.
pub fn took_of(body: &[u8]) -> Option<u32> {
    (body.len() == 4).then(|| u32::from_le_bytes([body[0], body[1], body[2], body[3]]))
}
