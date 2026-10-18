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

use crate::protocol::{
    decode_request, E_ACCES, E_INVAL, HDR_LEN, MAX_RW_PAYLOAD_BYTES, RESP_HDR_LEN, RW_HEADER_LEN,
    STATUS_LEN,
};
use crate::server::acl;
use crate::server::dispatch::dispatch;
use crate::server::error::{reply_decode_failed, reply_with_status};
use crate::server::settle::settle;
use crate::server::streak::Streak;
use crate::setup::Driver;
use alloc::vec;
use nonos_libc::{mk_getpid, mk_ipc_recv_from, mk_yield};

pub fn run(driver: &mut Driver) -> ! {
    let rx_len = HDR_LEN + RW_HEADER_LEN + MAX_RW_PAYLOAD_BYTES as usize;
    let tx_len = RESP_HDR_LEN + STATUS_LEN + MAX_RW_PAYLOAD_BYTES as usize;
    let mut rx = vec![0u8; rx_len];
    let mut tx = vec![0u8; tx_len];
    // This same loop drains the reply endpoint we answer on, so a reply that
    // finds no waiting caller comes straight back to us as an undecodable
    // message. Answering that failure sends another reply, which loops again:
    // one stray reply becomes a self-sustaining flood that buries every real
    // request. Drop anything we sent to ourselves before it can start the loop.
    let self_pid = mk_getpid();
    let mut streak = Streak::default();
    loop {
        let mut sender_pid: u32 = 0;
        let n = mk_ipc_recv_from(0, rx.as_mut_ptr(), rx_len, 0, &mut sender_pid);
        if !nonos_libc::recv_ready(n) {
            mk_yield();
            continue;
        }
        if sender_pid == self_pid {
            continue;
        }
        let len = n as usize;
        let Some(req) = decode_request(&rx[..len]) else {
            reply_decode_failed(&mut tx, E_INVAL);
            continue;
        };
        if !acl::permits(req.op, sender_pid) {
            let _ = reply_with_status(&mut tx, &req, E_ACCES);
            continue;
        }
        dispatch(driver, &req, &rx[HDR_LEN..len], &mut tx);
        // A request the device still holds (it timed out, or the device
        // answered out of turn) is ended by a reset now, after its caller
        // has its answer, so the next request finds a fresh device; or,
        // after too many resets in a row, the driver ends (`streak`).
        settle(driver, &mut streak);
    }
}
