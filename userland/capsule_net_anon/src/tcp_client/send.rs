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

//! Writing every byte, re-offering what a full socket did not take.

extern crate alloc;

use alloc::vec;
use nonos_libc::{mk_uptime_ms, mk_yield};

use super::envelope::call;
use super::errno::{E_LEN, E_SHORT_WRITE};

const OP_SEND: u16 = 5;
const SEGMENT_MAX: usize = 1460;
const SEND_DEADLINE_MS: i64 = 10_000;

/// Write every byte, offering again whatever the socket did not take.
///
pub fn send_all(port: u32, handle: u32, payload: &[u8]) -> Result<(), u16> {
    let deadline = mk_uptime_ms().saturating_add(SEND_DEADLINE_MS);
    let mut sent = 0usize;
    while sent < payload.len() {
        let end = (sent + SEGMENT_MAX).min(payload.len());
        let took = send_chunk(port, handle, &payload[sent..end])?;
        if took == 0 {
            if mk_uptime_ms() >= deadline {
                return Err(E_SHORT_WRITE);
            }
            mk_yield();
            continue;
        }
        sent += took;
    }
    Ok(())
}

fn send_chunk(port: u32, handle: u32, chunk: &[u8]) -> Result<usize, u16> {
    let mut body = vec![0u8; 4 + chunk.len()];
    body[0..4].copy_from_slice(&handle.to_le_bytes());
    body[4..].copy_from_slice(chunk);
    let mut out = [0u8; 4];
    if call(port, OP_SEND, &body, &mut out)? != 4 {
        return Err(E_LEN);
    }
    let took = u32::from_le_bytes(out) as usize;
    if took > chunk.len() {
        return Err(E_LEN);
    }
    Ok(took)
}
