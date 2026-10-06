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
use super::poll_irq::poll_irq;
use crate::protocol::{
    decode_request, E_ACCES, E_INVAL, HDR_LEN, MAX_RW_PAYLOAD_BYTES, OP_CONTROLLER_INFO,
    OP_HEALTHCHECK, OP_IDENTIFY, OP_PORT_LIST, RESP_HDR_LEN, RW_HEADER_LEN, SERVICE_NAME,
    STATUS_LEN,
};
use crate::served::Served;
use crate::server::{error, handlers};

pub fn run(served: &mut Served) -> ! {
    let rx_len = HDR_LEN + RW_HEADER_LEN + MAX_RW_PAYLOAD_BYTES as usize;
    let tx_len = RESP_HDR_LEN + STATUS_LEN + MAX_RW_PAYLOAD_BYTES as usize;
    let mut rx = vec![0u8; rx_len];
    let mut tx = vec![0u8; tx_len];
    let mut last_irq_seq = 0u64;
    let _service_name = SERVICE_NAME;

    loop {
        if let Served::Sata(driver) = served {
            poll_irq(driver, &mut last_irq_seq);
        }
        let mut sender_pid = 0u32;
        let n = mk_ipc_recv_from(0, rx.as_mut_ptr(), rx_len, 0, &mut sender_pid);
        if !nonos_libc::recv_ready(n) {
            continue;
        }
        let req = match decode_request(&rx[..n as usize]) {
            Some(r) => r,
            None => {
                error::reply_decode_failed(&mut tx, E_INVAL);
                continue;
            }
        };
        if !super::medium::permits(req.op, sender_pid) {
            error::reply_with_status(&mut tx, &req, E_ACCES);
            continue;
        }
        let body_end = n as usize;
        let body = &rx[HDR_LEN..body_end];
        let bare =
            matches!(req.op, OP_HEALTHCHECK | OP_CONTROLLER_INFO | OP_PORT_LIST | OP_IDENTIFY);
        if bare && req.payload_len != 0 {
            error::reply_with_status(&mut tx, &req, E_INVAL);
            continue;
        }
        match served {
            Served::Sata(driver) => dispatch(driver, &req, body, &mut tx),
            Served::Emmc(opened) => handlers::emmc::dispatch(opened, &req, body, &mut tx),
        }
    }
}
