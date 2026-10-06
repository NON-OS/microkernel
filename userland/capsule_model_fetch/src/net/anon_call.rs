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

/*
 * One request to `net.anon`: a 20-byte header (magic, version, operation,
 * status, request id, body length, little-endian), then the body; the
 * answer comes back the same way, its status where the request had none.
 */

use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::mk_ipc_call_timeout;

const MAGIC: u32 = 0x414E_4F31;
const VERSION: u16 = 1;
const HDR: usize = 20;
const PAYLOAD_MAX: usize = 32 * 1024;
/* A circuit's round trip crosses three relays. */
const CALL_MS: u64 = 20_000;

pub const OPEN: u16 = 5;
pub const SEND: u16 = 6;
pub const RECV: u16 = 7;
pub const CLOSE: u16 = 8;
pub const OK: u16 = 0;
pub const RX_EMPTY: u16 = 14;
pub const WOULD_BLOCK: u16 = 20;
/* Still building: no directory, no path, no link, no circuit, a stale one. */
pub const NOT_YET: [u16; 5] = [6, 7, 8, 10, 18];

/* The answer's status and body. */
pub fn call(port: u32, op: u16, body: &[u8]) -> Result<(u16, Vec<u8>), ()> {
    let mut req = Vec::with_capacity(HDR + body.len());
    req.extend_from_slice(&MAGIC.to_le_bytes());
    req.extend_from_slice(&VERSION.to_le_bytes());
    req.extend_from_slice(&op.to_le_bytes());
    req.extend_from_slice(&[0; 8]);
    req.extend_from_slice(&(body.len() as u32).to_le_bytes());
    req.extend_from_slice(body);
    let mut rx = vec![0u8; HDR + PAYLOAD_MAX];
    let n = mk_ipc_call_timeout(
        port as u64,
        req.as_ptr(),
        req.len(),
        rx.as_mut_ptr(),
        rx.len(),
        CALL_MS,
    );
    if n < HDR as i64 || rx[..4] != MAGIC.to_le_bytes() {
        return Err(());
    }
    let status = u16::from_le_bytes([rx[8], rx[9]]);
    let len = u32::from_le_bytes([rx[16], rx[17], rx[18], rx[19]]) as usize;
    let end = HDR.saturating_add(len).min(n as usize);
    Ok((status, rx[HDR..end].to_vec()))
}
