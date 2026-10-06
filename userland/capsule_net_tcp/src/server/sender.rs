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

use crate::state::Entry;
use crate::tcp::{window, FLAG_ACK, FLAG_PSH, MSS};

/// The peer's MSS once its SYN has been read; ours before that.
fn segment_max(e: &Entry) -> usize {
    match e.tcb.send.mss {
        0 => MSS,
        mss => usize::from(mss),
    }
}

pub fn drain_send(e: &mut Entry) {
    loop {
        let usable = window::usable(
            e.tcb.send.una,
            e.tcb.send.nxt,
            e.tcb.send.wnd as u32,
            e.cc.cwnd(),
        );
        if usable == 0 || e.snd_buf.is_empty() {
            break;
        }
        let n = (usable as usize).min(segment_max(e)).min(e.snd_buf.len());
        let seg_seq = e.tcb.send.nxt;
        let chunk: alloc::vec::Vec<u8> = e.snd_buf.drain(..n).collect();
        let _ = crate::server::tcp_tx::send(e.tcb, FLAG_ACK | FLAG_PSH, &chunk);
        e.tcb.send.nxt = e.tcb.send.nxt.wrapping_add(n as u32);
        e.retx_push(seg_seq, FLAG_ACK | FLAG_PSH, chunk);
    }
}
