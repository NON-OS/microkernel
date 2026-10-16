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
//! driver.gpio0's EVENTS (op 1), as F-57 gives its wire: a raw request,
//! the op byte then a device key u64 LE; the reply a status i32 LE, then a
//! count u8 and that many (controller `_UID` u16, pin u16) entries, oldest
//! first. Draining is destructive: an event is given once. Pure, for the
//! proofs; events.rs makes the call.

use alloc::vec::Vec;

pub const OP_EVENTS: u8 = 1;
/// The most entries one reply holds.
pub const EVENTS_MAX: usize = 100;
pub const REPLY_MAX: usize = 4 + 1 + EVENTS_MAX * 4;

pub fn request(key: u64) -> [u8; 9] {
    let mut r = [0u8; 9];
    r[0] = OP_EVENTS;
    r[1..].copy_from_slice(&key.to_le_bytes());
    r
}

/// The events of a reply, or the status it refused with; `None` for a
/// reply shorter than its own count says.
pub fn events(reply: &[u8]) -> Option<Result<Vec<(u16, u16)>, i32>> {
    let status = i32::from_le_bytes(reply.get(..4)?.try_into().ok()?);
    if status != 0 {
        return Some(Err(status));
    }
    let count = *reply.get(4)? as usize;
    let body = reply.get(5..5 + count * 4)?;
    let word = |c: &[u8], at: usize| u16::from_le_bytes([c[at], c[at + 1]]);
    Some(Ok(body.chunks(4).map(|c| (word(c, 0), word(c, 2))).collect()))
}

/// Whether the touchpad's line, `uid` and `pin`, is among `events`.
pub fn rang(events: &[(u16, u16)], uid: u16, pin: u16) -> bool {
    events.contains(&(uid, pin))
}
