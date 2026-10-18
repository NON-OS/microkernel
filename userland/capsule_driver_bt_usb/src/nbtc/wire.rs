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

//! NBTC v1, the protocol driver.bt_usb0 answers. The frame is NNET's
//! (nonos_usbnet/src/nnet/wire.rs): a 20-byte header of magic, version, op,
//! flags, a zero half-word, the request id and the payload's length; in a
//! reply, a status word and then the op's payload, both counted in the length.

use nonos_bt_bond::MAX_BONDS;
use nonos_bt_proto::{BONDS_MAX, LEVELS_MAX, LEVEL_LEN};
pub use nonos_bt_proto::{BOND_LEN, DATA_AT, HDR_LEN, MAGIC, STATUS_LEN, VERSION};

/// The longest reply: the bond list when every place is taken.
pub const REPLY_MAX: usize = DATA_AT + 1 + BOND_LEN * MAX_BONDS;

// A client sizes its lists by the shared crate's bounds: the store keeps
// no more bonds than those name, and BATTERY at its fullest fits a reply.
const _: () = assert!(MAX_BONDS <= BONDS_MAX);
const _: () = assert!(DATA_AT + LEVEL_LEN * LEVELS_MAX < REPLY_MAX);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Request {
    pub op: u16,
    pub flags: u16,
    pub request_id: u32,
}

/// The request and its payload. A frame that is not one is the error: what
/// could be read of its header, so the reply still names the request.
pub fn decode(buf: &[u8]) -> Result<(Request, &[u8]), Request> {
    if buf.len() < HDR_LEN {
        return Err(Request::default());
    }
    let word = |at: usize| u32::from_le_bytes([buf[at], buf[at + 1], buf[at + 2], buf[at + 3]]);
    let half = |at: usize| u16::from_le_bytes([buf[at], buf[at + 1]]);
    let req = Request { op: half(6), flags: half(8), request_id: word(12) };
    let len = word(16) as usize;
    if word(0) != MAGIC || half(4) != VERSION || half(10) != 0 || len != buf.len() - HDR_LEN {
        return Err(req);
    }
    Ok((req, &buf[HDR_LEN..]))
}

/// The header and `status`, with `data_len` payload octets the caller has
/// written at `DATA_AT`. The reply's length.
pub fn reply(tx: &mut [u8], req: &Request, status: i32, data_len: usize) -> usize {
    let plen = (STATUS_LEN + data_len) as u32;
    tx[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    tx[4..6].copy_from_slice(&VERSION.to_le_bytes());
    tx[6..8].copy_from_slice(&req.op.to_le_bytes());
    tx[8..10].copy_from_slice(&req.flags.to_le_bytes());
    tx[10..12].fill(0);
    tx[12..16].copy_from_slice(&req.request_id.to_le_bytes());
    tx[16..20].copy_from_slice(&plen.to_le_bytes());
    tx[HDR_LEN..DATA_AT].copy_from_slice(&status.to_le_bytes());
    DATA_AT + data_len
}

/// A reply carrying only `status`.
pub fn status(tx: &mut [u8], req: &Request, status: i32) -> usize {
    reply(tx, req, status, 0)
}
