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

use super::constants::{PROTOCOL_VERSION, REQ_CONNECT, REQ_SEND};
use super::hostport::write_hostport;
use super::provider::{open_envelope, ENVELOPE_BYTES};
use crate::conn::Dest;

/// Encode a connect request naming `dest`, with no return address: replies
/// come back through reply blocks, so nothing here identifies the sender.
/// Returns the byte count, or `None` if `out` is too small for the address.
pub fn encode_connect(conn_id: u64, dest: &Dest, out: &mut [u8]) -> Option<usize> {
    // [envelope][version][flag][conn_id:8][addr_len:2][addr]
    let base = open_envelope(out)?;
    let body = out.get_mut(base..)?;
    if body.len() < 2 + 8 + 2 {
        return None;
    }
    body[0] = PROTOCOL_VERSION;
    body[1] = REQ_CONNECT;
    body[2..10].copy_from_slice(&conn_id.to_be_bytes());
    let addr_len = write_hostport(dest, body.get_mut(12..)?)?;
    body[10..12].copy_from_slice(&(addr_len as u16).to_be_bytes());
    Some(base + 12 + addr_len)
}

/// The send header after the envelope: version, flag, conn_id, closed, seq.
const SEND_HEADER_BYTES: usize = 2 + 8 + 1 + 8;

/// The longest send frame net.nym takes: one mix payload. It refuses a longer
/// one whole.
pub const SEND_FRAME_MAX: usize = 1024;

/// The most stream bytes one send frame carries; a longer write goes as
/// several sends, each with its own number.
pub const SEND_DATA_MAX: usize = SEND_FRAME_MAX - ENVELOPE_BYTES - SEND_HEADER_BYTES;

/// Encode a send request carrying `data` for `conn_id` at stream position
/// `seq`. `closed` marks our half of the stream finished, and an empty closing
/// send is how a connection is torn down. Returns the byte count, or `None` if
/// `out` is too small.
pub fn encode_send(
    conn_id: u64,
    seq: u64,
    closed: bool,
    data: &[u8],
    out: &mut [u8],
) -> Option<usize> {
    // [envelope][version][flag][conn_id:8][closed:1][seq:8][data]
    let base = open_envelope(out)?;
    let body = out.get_mut(base..)?;
    let total = SEND_HEADER_BYTES + data.len();
    if body.len() < total {
        return None;
    }
    body[0] = PROTOCOL_VERSION;
    body[1] = REQ_SEND;
    body[2..10].copy_from_slice(&conn_id.to_be_bytes());
    body[10] = closed as u8;
    body[11..19].copy_from_slice(&seq.to_be_bytes());
    body[19..total].copy_from_slice(data);
    Some(base + total)
}
