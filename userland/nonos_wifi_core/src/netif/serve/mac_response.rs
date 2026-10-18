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

//! Encoders for the replies net_core reads: MAC address and received
//! frames.

use super::*;

// Response: header + i32 status + 6-byte MAC. Unassociated yields a no-link
// status with a zeroed address, which net_core reads as "no MAC yet".
pub(super) fn mac_response(out: &mut [u8], rid: u32, mac: Option<[u8; 6]>) -> Option<usize> {
    let n = write_response_header(out, OP_MAC_ADDRESS, rid, 4 + 6)?;
    let (status, addr) = match mac {
        Some(m) => (0, m),
        None => (-STATUS_NO_LINK, [0u8; 6]),
    };
    put_i32(out, n, status)?;
    out.get_mut(n + 4..n + 10)?.copy_from_slice(&addr);
    Some(n + 10)
}

// Response: header + i32 status + u32 frame_len + frame. An empty poll is a
// retry status with no frame, exactly what net_core drains against.
pub(super) fn rx_response<L: LinkPort>(out: &mut [u8], rid: u32, link: &mut L) -> Option<usize> {
    let mut frame = [0u8; MAX_FRAME];
    match link.poll_rx(&mut frame) {
        Some(len) if len <= MAX_FRAME => {
            let n = write_response_header(out, OP_RX_PACKET, rid, (4 + 4 + len) as u32)?;
            put_i32(out, n, 0)?;
            out.get_mut(n + 4..n + 8)?.copy_from_slice(&(len as u32).to_le_bytes());
            out.get_mut(n + 8..n + 8 + len)?.copy_from_slice(&frame[..len]);
            Some(n + 8 + len)
        }
        _ => {
            let n = write_response_header(out, OP_RX_PACKET, rid, 4)?;
            put_i32(out, n, STATUS_AGAIN)?;
            Some(n + 4)
        }
    }
}

pub(super) fn put_i32(out: &mut [u8], at: usize, val: i32) -> Option<()> {
    out.get_mut(at..at + 4)?.copy_from_slice(&val.to_le_bytes());
    Some(())
}

/// The largest response this adapter ever writes: an RX response carrying a
/// full frame. Callers size their output buffer with this.
pub const MAX_RESPONSE: usize = HDR_LEN + 4 + 4 + MAX_FRAME;
