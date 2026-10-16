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

//! The control family's replies: the status, and a scan from what was heard.

use super::kinds::{
    View, E_NO_OP, OP_SCAN, OP_STATUS, REPLY_MAX, SCAN_STATS, STATUS_LEN, WIFI_HDR, WIFI_MAGIC,
};

fn put32(out: &mut [u8], at: usize, v: u32) {
    out[at..at + 4].copy_from_slice(&v.to_le_bytes());
}

/// Answer a control request into `out` (at least `REPLY_MAX` bytes). `None`
/// when `req` is not one, so it goes on to this driver's own protocol.
pub fn answer(req: &[u8], view: &View<'_>, out: &mut [u8]) -> Option<usize> {
    if req.len() < WIFI_HDR || out.len() < REPLY_MAX {
        return None;
    }
    if u32::from_le_bytes([req[0], req[1], req[2], req[3]]) != WIFI_MAGIC {
        return None;
    }
    out[..WIFI_HDR].copy_from_slice(&req[..WIFI_HDR]);
    let op = u16::from_le_bytes([req[4], req[5]]);
    let n = match (op, &view.heard) {
        (OP_STATUS, heard) => {
            out[WIFI_HDR] = view.stage;
            out[WIFI_HDR + 1] = view.step;
            let (sweeps, stalls, frames, beacons) =
                heard.as_ref().map_or((0, 0, 0, 0), |h| (h.sweeps, h.stalls, h.frames, h.beacons));
            let words = [view.detail, view.hw_rev, view.rf_id, sweeps, stalls, frames, beacons];
            for (i, w) in words.iter().enumerate() {
                put32(out, WIFI_HDR + 2 + 4 * i, *w);
            }
            STATUS_LEN
        }
        (OP_SCAN, Some(h)) => {
            put32(out, WIFI_HDR, h.sweeps);
            put32(out, WIFI_HDR + 4, h.frames);
            put32(out, WIFI_HDR + 8, h.beacons);
            SCAN_STATS + h.list.encode(&mut out[WIFI_HDR + SCAN_STATS..])
        }
        _ => {
            out[WIFI_HDR..WIFI_HDR + 4].copy_from_slice(&E_NO_OP.to_le_bytes());
            4
        }
    };
    Some(WIFI_HDR + n)
}
