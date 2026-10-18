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

//! The capsule's requests, raw like capsule_acpi's: an op byte and its
//! arguments; a status word and a body back. Every answer about a device
//! is given only to a sender holding it (capsule_acpi's HOLDER, the
//! kernel's claim test).
//!
//! - EVENTS (1, device key u64): its waiting events, a count u8 then
//!   (controller `_UID` u16, pin u16) each, oldest first.
//! - WRITE (2) and REARM (3): `pin_requests`.

use crate::acpi::holds;
use crate::bank::Bank;
use crate::pin_requests::{rearm, write};
use crate::queue::Queue;
use alloc::vec::Vec;

pub const EPERM: i32 = -1;
pub const ENOENT: i32 = -2;
pub const EIO: i32 = -5;
pub const EINVAL: i32 = -22;
const MAX_EVENTS: usize = 100;

pub fn word(b: &[u8], at: usize) -> Option<u16> {
    Some(u16::from_le_bytes(b.get(at..at + 2)?.try_into().ok()?))
}

pub fn reply(status: i32, body: &[u8]) -> Vec<u8> {
    let mut v = status.to_le_bytes().to_vec();
    v.extend_from_slice(body);
    v
}

fn events(queue: &mut Queue, req: &[u8], sender: u32) -> Vec<u8> {
    let Some(key) = req.get(1..9).and_then(|b| b.try_into().ok()).map(u64::from_le_bytes) else {
        return reply(EINVAL, &[]);
    };
    if !holds(sender, key) {
        return reply(EPERM, &[]);
    }
    let waiting = queue.drain(key, MAX_EVENTS);
    let mut body = alloc::vec![waiting.len() as u8];
    for (uid, pin) in waiting {
        body.extend_from_slice(&uid.to_le_bytes());
        body.extend_from_slice(&pin.to_le_bytes());
    }
    reply(0, &body)
}

pub fn answer(banks: &mut [Bank], queue: &mut Queue, req: &[u8], sender: u32) -> Vec<u8> {
    match req.first() {
        Some(1) => events(queue, req, sender),
        Some(2) => reply(write(banks, req, sender), &[]),
        Some(3) => reply(rearm(banks, req, sender), &[]),
        _ => reply(EINVAL, &[]),
    }
}
