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
//! The host wire, as every USB host serves it: a 20-byte header (magic,
//! version, op, request id, payload length), then the body; a reply's
//! payload starts with its status.

use core::sync::atomic::{AtomicU32, Ordering};

use nonos_libc::mk_ipc_call;

pub const HDR_LEN: usize = 20;
const MAGIC: u32 = 0x4E58_4843;
const VERSION: u16 = 1;
/// The wire's I/O error, for a call that brought no reply.
pub const E_IO: i32 = -5;

pub const OP_PORT_STATUS: u16 = 0x0003;
pub const OP_ENABLE_SLOT: u16 = 0x0004;
pub const OP_DISABLE_SLOT: u16 = 0x0005;
pub const OP_ADDRESS_DEVICE: u16 = 0x0006;
pub const OP_ALLOC_TRANSFER_RING: u16 = 0x0009;
pub const OP_CONTROL_TRANSFER: u16 = 0x000B;
pub const OP_ADDRESS_ROUTED: u16 = 0x0020;
pub const OP_CONFIGURE_HUB: u16 = 0x0021;

/// A control request's head and up to 512 data bytes.
const BODY_MAX: usize = 10 + 512;
/// A port list's head, 255 entries and their extensions.
const REPLY_MAX: usize = 4 + 16 * 255;

static SEQ: AtomicU32 = AtomicU32::new(1);

/// One call to the host at `port`: the reply's status and its data, put
/// at the front of `data` (none past its length).
pub fn call(port: u32, op: u16, body: &[u8], data: &mut [u8]) -> (i32, usize) {
    // A request that does not fit is refused here, before it is sent.
    let mut req = [0u8; HDR_LEN + BODY_MAX];
    let Some(out) = req.get_mut(..HDR_LEN + body.len()) else { return (E_IO, 0) };
    out[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    out[4..6].copy_from_slice(&VERSION.to_le_bytes());
    out[6..8].copy_from_slice(&op.to_le_bytes());
    out[12..16].copy_from_slice(&SEQ.fetch_add(1, Ordering::Relaxed).to_le_bytes());
    out[16..20].copy_from_slice(&(body.len() as u32).to_le_bytes());
    out[HDR_LEN..].copy_from_slice(body);
    let mut resp = [0u8; HDR_LEN + 4 + REPLY_MAX];
    let n = mk_ipc_call(port as u64, out.as_ptr(), out.len(), resp.as_mut_ptr(), resp.len());
    let ok = n >= (HDR_LEN + 4) as i64 && resp[0..4] == MAGIC.to_le_bytes();
    if !ok {
        return (E_IO, 0);
    }
    let payload = u32::from_le_bytes([resp[16], resp[17], resp[18], resp[19]]) as usize;
    let status = i32::from_le_bytes([resp[20], resp[21], resp[22], resp[23]]);
    let have = payload.saturating_sub(4).min(resp.len() - HDR_LEN - 4).min(data.len());
    data[..have].copy_from_slice(&resp[HDR_LEN + 4..HDR_LEN + 4 + have]);
    (status, have)
}
