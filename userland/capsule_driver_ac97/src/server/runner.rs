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

//! The loop: refill while a stream runs, then serve what is queued. After
//! one request the ones already waiting are served too, each with a short
//! wait, before the next refill: the server writes a period in 4 KiB pieces
//! and one piece per look fed the engine half of what it played (the HD
//! Audio driver learned this). The bound keeps a flood of requests from
//! holding off the refill.

use alloc::vec;

use nonos_libc::mk_ipc_recv_from;

use super::{dispatch::serve, refill::refill, stream::Stream};
use crate::protocol::{HDR_LEN, MAX_PCM_CHUNK, OUTPUT_STATUS_LEN, STATUS_LEN};
use crate::setup::Driver;

const RX_LEN: usize = HDR_LEN + MAX_PCM_CHUNK;
const TX_LEN: usize = HDR_LEN + STATUS_LEN + OUTPUT_STATUS_LEN;
const RECV_TIMEOUT_MS: u64 = 5;
const DRAIN_WAIT_MS: u64 = 1;
const DRAIN_MAX: usize = 16;

pub fn run(d: &mut Driver) -> ! {
    let mut rx = vec![0u8; RX_LEN];
    let mut tx = [0u8; TX_LEN];
    let mut st = Stream::default();
    loop {
        if st.running {
            refill(d, &mut st);
        }
        let mut wait = RECV_TIMEOUT_MS;
        for _ in 0..DRAIN_MAX {
            let mut sender = 0u32;
            let n = mk_ipc_recv_from(0, rx.as_mut_ptr(), RX_LEN, wait, &mut sender);
            if !nonos_libc::recv_ready(n) {
                break;
            }
            wait = DRAIN_WAIT_MS;
            if sender != 0 {
                serve(d, &mut st, sender, &rx[..(n as usize).min(RX_LEN)], &mut tx);
            }
        }
    }
}
