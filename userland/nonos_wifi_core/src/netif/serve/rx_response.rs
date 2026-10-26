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

//! The reply to OP_RX_PACKET: the frame kept for net.core's number, or a
//! fresh one.

use super::mac_response::put_i32;
use super::*;
use nonos_mac::resend::{asked, serve};
use nonos_mac::resend_cell::KeptCell;

// The frame handed out last, kept until net.core asks the next number: a
// reply that came too late is asked for again and not lost (nonos_mac::resend).
static KEPT: KeptCell<MAX_FRAME> = KeptCell::new();

// Response: header + i32 status + u32 frame_len + frame. An empty poll is a
// retry status with no frame, exactly what net_core drains against.
pub(super) fn rx_response<L: LinkPort>(
    out: &mut [u8],
    (rid, body): (u32, &[u8]),
    link: &mut L,
) -> Option<usize> {
    let mut frame = [0u8; MAX_FRAME];
    let len = KEPT
        .with(|k| {
            let got = serve(k, asked(body), |b| Ok::<_, ()>(link.poll_rx(b))).ok().flatten()?;
            frame[..got.len()].copy_from_slice(got);
            Some(got.len())
        })
        .unwrap_or_else(|| link.poll_rx(&mut frame));
    match len {
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
