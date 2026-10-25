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

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};

use nonos_libc::mk_ipc_call_timeout;

use super::learned_bound::SINGLE;
use super::rx_learned::{timed, SINGLE_MS};
use super::rx_seq::next_rid;
use crate::protocol::header::{parse_response, write_request};
use crate::protocol::ops::{HDR_LEN, OP_RX_PACKET};

const MAX_FRAME: usize = 1514;
const RESP_CAP: usize = HDR_LEN + 4 + 4 + 12 + MAX_FRAME;

/// The frame number asked for next. It moves only on a reply read with a
/// frame in it: a driver keeps the frame it handed out under this number
/// (nonos_mac::resend), so a reply that came too late is asked for again,
/// not lost. A driver that reads no body serves a fresh frame, as before.
static FRAME_SEQ: AtomicU32 = AtomicU32::new(1);

pub fn poll_frame(port: u32) -> Option<Vec<u8>> {
    let seq = FRAME_SEQ.load(Ordering::Relaxed);
    let mut req = [0u8; HDR_LEN + 4];
    write_request(&mut req, OP_RX_PACKET, next_rid(), 4)?;
    req[HDR_LEN..].copy_from_slice(&seq.to_le_bytes());
    let mut resp = vec![0u8; RESP_CAP];
    let (rp, rl) = (resp.as_mut_ptr(), resp.len());
    let n = timed(&SINGLE_MS, &SINGLE, |ms| {
        mk_ipc_call_timeout(port as u64, req.as_ptr(), req.len(), rp, rl, ms)
    });
    let frame = frame_of(resp.get(..usize::try_from(n).ok()?)?)?.to_vec();
    FRAME_SEQ.store(seq.wrapping_add(1).max(1), Ordering::Relaxed);
    Some(frame)
}

/// The frame an OP_RX_PACKET reply carries: status 0, then its length and
/// bytes. Anything else, a retry status included, carries none.
fn frame_of(view: &[u8]) -> Option<&[u8]> {
    let (op, _, plen) = parse_response(view)?;
    let status = i32::from_le_bytes(view.get(HDR_LEN..HDR_LEN + 4)?.try_into().ok()?);
    if op != OP_RX_PACKET || status != 0 || (plen as usize) < 8 {
        return None;
    }
    let len = u32::from_le_bytes(view.get(HDR_LEN + 4..HDR_LEN + 8)?.try_into().ok()?) as usize;
    view.get(HDR_LEN + 8..HDR_LEN + 8 + len)
}
