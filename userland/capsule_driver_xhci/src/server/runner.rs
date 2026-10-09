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
    decode_request, BULK_HEADER_LEN, BULK_MAX, E_INVAL, HDR_LEN, MAX_PORTS_REPORTED,
    MAX_REQUEST_PAYLOAD_LEN, PORT_ENTRY_BYTES, PORT_STATUS_HEADER_BYTES, RESP_HDR_LEN, STATUS_LEN,
};
use crate::server::context::Context;
use crate::server::error::{reply_decode_failed, reply_with_status};
use crate::server::mux::Mux;
use crate::server::service_interrupts::service_interrupts;
use crate::setup::Driver;
use alloc::vec;
use alloc::vec::Vec;
use nonos_libc::mk_ipc_recv_from;
const PORTS_REPLY: usize =
    RESP_HDR_LEN + STATUS_LEN + PORT_STATUS_HEADER_BYTES + MAX_PORTS_REPORTED * PORT_ENTRY_BYTES;
const BULK_REPLY: usize = RESP_HDR_LEN + STATUS_LEN + BULK_HEADER_LEN + BULK_MAX;
const TX_LEN: usize = if PORTS_REPLY > BULK_REPLY { PORTS_REPLY } else { BULK_REPLY };
/// Serve `drivers`, the primary controller first.
pub fn run(drivers: Vec<Driver>) -> ! {
    let mut rx = vec![0u8; HDR_LEN + MAX_REQUEST_PAYLOAD_LEN];
    let mut tx = vec![0u8; TX_LEN];
    let mut mux = Mux::new(drivers.into_iter().map(Context::new).collect());
    loop {
        for ctx in mux.ctxs.iter_mut() {
            service_interrupts(ctx);
        }
        let mut sender_pid: u32 = 0;
        let n = mk_ipc_recv_from(0, rx.as_mut_ptr(), rx.len(), 0, &mut sender_pid);
        crate::server::reply::set_sender(sender_pid);
        if !nonos_libc::recv_ready(n) {
            continue;
        }
        let req = match decode_request(&rx[..n as usize]) {
            Some(r) => r,
            None => {
                reply_decode_failed(&mut tx, E_INVAL);
                continue;
            }
        };
        let len = n as usize;
        let expected = HDR_LEN + req.payload_len as usize;
        if expected != len || req.payload_len as usize > MAX_REQUEST_PAYLOAD_LEN {
            reply_with_status(&mut tx, &req, E_INVAL);
            continue;
        }
        let body = &rx[HDR_LEN..len];
        mux.dispatch(&req, body, &mut tx);
    }
}
