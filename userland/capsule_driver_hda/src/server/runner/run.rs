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

use alloc::vec;

use nonos_libc::mk_ipc_recv;

use super::jack_poll::JackPoll;
use super::max_tx_body::max_tx_body;
use super::poll_irq::poll_irq;
use super::refill::Refill;
use super::serve::serve;
use super::stream::Stream;
use crate::audio::PcmQueue;
use crate::protocol::{HDR_LEN, MAX_PCM_CHUNK, RESP_HDR_LEN, SERVICE_NAME};
use crate::setup::Driver;

const TX_LEN: usize = RESP_HDR_LEN + 4 + max_tx_body();
const RX_LEN: usize = HDR_LEN + MAX_PCM_CHUNK;
const RECV_TIMEOUT_MS: u64 = 5;
/*
 * After one request is served, the ones already queued behind it are served
 * too before the next interrupt wait, each with a short wait of its own. A
 * period is 8 KiB, about 43 ms at 48 kHz, and the audio server writes it in
 * 4 KiB pieces; serving one piece per interrupt fed the controller half of
 * what it played, and every stream ran dry. The bound keeps a flood of
 * requests from holding off the refill.
 */
const DRAIN_WAIT_MS: u64 = 1;
const DRAIN_MAX: usize = 16;

pub fn run(mut driver: Driver) -> ! {
    let mut rx = vec![0u8; RX_LEN];
    let mut tx = vec![0u8; TX_LEN];
    let mut last_irq_seq = 0u64;
    let rf = Refill::new(driver.posbuf_va.is_some());
    let mut st = Stream { played: false, running: false, q: PcmQueue::new(), rf };
    let mut jack = JackPoll::new();
    let _service_name = SERVICE_NAME;
    loop {
        jack.poll(&mut driver);
        #[cfg(feature = "nonos-driver-hda-smoketest")]
        if st.running {
            super::selftest::feed_idle(&mut st.q);
        }
        poll_irq(&driver, &mut last_irq_seq, &mut st.played, &mut st.q, &mut st.rf, st.running);
        let mut wait = RECV_TIMEOUT_MS;
        for _ in 0..DRAIN_MAX {
            let n = mk_ipc_recv(0, rx.as_mut_ptr(), RX_LEN, wait);
            if !nonos_libc::recv_ready(n) {
                break;
            }
            serve(&mut driver, &rx[..n as usize], &mut tx, &mut st);
            wait = DRAIN_WAIT_MS;
        }
    }
}
