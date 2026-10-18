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

use nonos_libc::mk_ipc_recv_from;

use super::dispatch::dispatch;
use super::{medium, reply};
use crate::platform::Opened;
use crate::protocol::{
    body_fits, decode_request, E_ACCES, E_INVAL, HDR_LEN, RESP_HDR_LEN, RW_HEADER_LEN, STATUS_LEN,
};

pub fn run(o: &mut Opened) -> ! {
    // One request's largest data, a READ reply or a WRITE body.
    let most = o.ufs.max_bytes();
    let rx_len = HDR_LEN + RW_HEADER_LEN + most;
    let mut rx = vec![0u8; rx_len];
    let mut tx = vec![0u8; RESP_HDR_LEN + STATUS_LEN + most];
    loop {
        let mut sender_pid = 0u32;
        let n = mk_ipc_recv_from(0, rx.as_mut_ptr(), rx_len, 0, &mut sender_pid);
        if !nonos_libc::recv_ready(n) {
            continue;
        }
        let Some(req) = decode_request(&rx[..n as usize]) else {
            reply::decode_failed(&mut tx, E_INVAL);
            continue;
        };
        if !medium::permits(req.op, sender_pid) {
            reply::status(&mut tx, &req, E_ACCES);
            continue;
        }
        let body = &rx[HDR_LEN..n as usize];
        if !body_fits(&req, body.len()) {
            reply::status(&mut tx, &req, E_INVAL);
            continue;
        }
        dispatch(o, &req, body, &mut tx);
    }
}
