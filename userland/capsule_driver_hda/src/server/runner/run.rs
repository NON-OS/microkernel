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

use super::max_tx_body::max_tx_body;
use super::poll_irq::poll_irq;
use super::refill::Refill;
use crate::audio::PcmQueue;
use super::jack_poll::JackPoll;
use crate::protocol::{
    decode_request, refused, E_INVAL, HDR_LEN, MAX_PCM_CHUNK, OP_CODEC_LIST, OP_CODEC_MASK,
    OP_CONTROLLER_INFO, OP_HEALTHCHECK, OP_OUTPUT_STATUS, OP_PLAY_TONE, OP_STREAM_LAYOUT,
    OP_STREAM_START, OP_STREAM_STOP, OP_WRITE_PCM, RESP_HDR_LEN, SERVICE_NAME,
};
use crate::server::{error, handlers};
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

struct Stream {
    played: bool,
    running: bool,
    q: PcmQueue,
    rf: Refill,
}

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

fn serve(driver: &mut Driver, msg: &[u8], tx: &mut [u8], st: &mut Stream) {
    let req = match decode_request(msg) {
        Some(r) => r,
        None => {
            error::reply_with_status(tx, &refused(msg), E_INVAL);
            return;
        }
    };
    if req.payload_len != 0 && req.op != OP_WRITE_PCM {
        error::reply_with_status(tx, &req, E_INVAL);
        return;
    }
    match req.op {
        OP_HEALTHCHECK => handlers::health::handle(&req, tx),
        OP_CONTROLLER_INFO => handlers::controller_info::handle(driver, &req, tx),
        OP_CODEC_MASK => handlers::codec_mask::handle(driver, &req, tx),
        OP_STREAM_LAYOUT => handlers::stream_layout::handle(driver, &req, tx),
        OP_CODEC_LIST => handlers::codec_list::handle(driver, &req, tx),
        OP_OUTPUT_STATUS => handlers::output_status::handle(&driver.status, &req, tx),
        OP_PLAY_TONE => {
            let was = st.running;
            handlers::play_tone::handle(driver, &req, tx, &mut st.played, &mut st.running);
            if st.running && !was {
                st.q.clear();
                st.rf.reset();
            }
        }
        OP_STREAM_START => {
            let was = st.running;
            handlers::stream::handle_stream_start(driver, &req, tx, &mut st.running);
            if st.running && !was {
                st.rf.reset();
            }
        }
        OP_STREAM_STOP => {
            let was = st.running;
            handlers::stream::handle_stream_stop(driver, &req, tx, &mut st.q, &mut st.running);
            if was && !st.running {
                st.rf.report();
            }
        }
        OP_WRITE_PCM => handlers::write_pcm::handle(&mut st.q, &req, msg, tx),
        _ => error::reply_with_status(tx, &req, E_INVAL),
    }
}
