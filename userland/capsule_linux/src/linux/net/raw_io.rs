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

//! Bytes on a socket this capsule opened for itself.

use alloc::vec::Vec;

use nonos_libc::{mk_yield, Deadline};

use super::call::call;
use super::ops::{OP_CLOSE, OP_RECV, OP_SEND};

/// One transfer; the service caps a reply, so a body arrives in pieces.
const CHUNK: usize = 32 << 10;

pub fn send_all(handle: u32, bytes: &[u8]) -> Option<()> {
    for part in bytes.chunks(CHUNK) {
        let mut body = Vec::with_capacity(4 + part.len());
        body.extend_from_slice(&handle.to_le_bytes());
        body.extend_from_slice(part);
        match call(OP_SEND, &body, 0) {
            Some((0, _)) => {}
            _ => return None,
        }
    }
    Some(())
}

/// Read until `done` says the reply is whole, or nothing arrives for
/// `idle_ms`. An empty read is "nothing yet": net.core answers the same for
/// a quiet socket and a closed one, so it cannot mean the end.
type Done = dyn Fn(&[u8]) -> bool;

pub fn recv_until(handle: u32, limit: usize, done: &Done, idle_ms: u64) -> Option<Vec<u8>> {
    let mut out: Vec<u8> = Vec::new();
    let mut quiet = Deadline::after_ms(idle_ms);
    loop {
        match call(OP_RECV, &handle.to_le_bytes(), CHUNK) {
            Some((0, part)) if !part.is_empty() => {
                out.extend_from_slice(&part);
                quiet = Deadline::after_ms(idle_ms);
            }
            _ if quiet.expired() => return Some(out),
            _ => {
                let _ = mk_yield();
                continue;
            }
        }
        if out.len() > limit {
            return None;
        }
        if done(&out) {
            return Some(out);
        }
    }
}

pub fn close(handle: u32) {
    let _ = call(OP_CLOSE, &handle.to_le_bytes(), 0);
}
