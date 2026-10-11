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

//! The serve loop: one request in, one reply out, to whichever capsule sent
//! it. The receive waits at most a tick, so the link and the firmware
//! heartbeat are seen to with no request coming in. A request whose header
//! does not decode, or whose length disagrees with its payload, is answered
//! with an error and nothing else is done.

use alloc::vec;

use nonos_libc::mk_ipc_recv_from;

use crate::constants::timeouts::TICK_MS;
use crate::constants::MAX_ETHERNET_FRAME;
use crate::protocol::{
    decode_request, E_INVAL, E_MSGSIZE, HDR_LEN, RESP_HDR_LEN, RX_PAYLOAD_PREFIX_LEN,
    STATS_PAYLOAD_LEN, STATUS_LEN,
};
use crate::server::dispatch::dispatch;
use crate::server::error::{reply_decode_failed, reply_with_status};
use crate::server::tick::Ticker;
use crate::setup::Driver;

const SERVICE_INBOX: u64 = 0;

pub fn run(driver: &mut Driver) -> ! {
    let rx_len = HDR_LEN + MAX_ETHERNET_FRAME;
    let tx_len = RESP_HDR_LEN
        + STATUS_LEN
        + core::cmp::max(RX_PAYLOAD_PREFIX_LEN + MAX_ETHERNET_FRAME, STATS_PAYLOAD_LEN);
    let mut rx = vec![0u8; rx_len];
    let mut tx = vec![0u8; tx_len];
    let mut ticker = Ticker::new();

    loop {
        let mut sender: u32 = 0;
        let n = mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx_len, TICK_MS, &mut sender);
        let ready = nonos_libc::recv_ready(n);
        ticker.run(driver);
        if !ready || sender == 0 {
            continue;
        }
        let len = n as usize;
        let Some(req) = decode_request(&rx[..len]) else {
            reply_decode_failed(sender, &mut tx, E_INVAL);
            continue;
        };
        let body_end = HDR_LEN.saturating_add(req.payload_len as usize);
        if body_end != len {
            reply_with_status(sender, &mut tx, &req, E_MSGSIZE);
            continue;
        }
        dispatch(sender, driver, &req, &rx[HDR_LEN..body_end], &mut tx);
    }
}
