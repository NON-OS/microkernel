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

//! The server loop: take each request, check it, and hand it on.

use alloc::vec;

use nonos_libc::{mk_getpid, mk_ipc_recv_from};

use super::{dispatch::dispatch, medium, reply};
use crate::constants::buffer::DATA_BYTES;
use crate::setup::Driver;
use crate::wire::{
    decode_request, Request, E_ACCES, E_INVAL, E_MSGSIZE, HDR_LEN, RESP_HDR_LEN, RW_HEADER_LEN,
    STATUS_LEN,
};

/// The largest request is a full write; the largest reply a full read.
pub const RX_LEN: usize = HDR_LEN + RW_HEADER_LEN + DATA_BYTES;
pub const TX_LEN: usize = RESP_HDR_LEN + STATUS_LEN + DATA_BYTES;

pub fn run(d: &mut Driver) -> ! {
    let mut rx = vec![0u8; RX_LEN];
    let mut tx = vec![0u8; TX_LEN];
    // This loop also drains the reply endpoint it answers on, so a reply
    // that found no waiting caller comes back here as a frame that is not a
    // request; answering it would send another, and one stray reply would
    // become a flood. Anything this driver sent itself is dropped.
    let self_pid = mk_getpid();
    loop {
        let mut sender = 0u32;
        let n = mk_ipc_recv_from(0, rx.as_mut_ptr(), RX_LEN, 0, &mut sender);
        if !nonos_libc::recv_ready(n) || sender == self_pid {
            continue;
        }
        serve(d, sender, &rx[..(n as usize).min(RX_LEN)], &mut tx);
    }
}

/// One frame from `sender`, answered.
pub fn serve(d: &mut Driver, sender: u32, msg: &[u8], tx: &mut [u8]) {
    let Some(req) = decode_request(msg) else {
        let none = Request { op: 0, flags: 0, unit: 0, request_id: 0, payload_len: 0 };
        return reply::status(tx, &none, E_INVAL);
    };
    if !medium::permits(req.op, sender) {
        return reply::status(tx, &req, E_ACCES);
    }
    let body = &msg[HDR_LEN..];
    if req.payload_len as usize != body.len() {
        return reply::status(tx, &req, E_MSGSIZE);
    }
    dispatch(d, &req, body, tx);
}
