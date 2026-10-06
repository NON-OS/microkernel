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
//! Serving a machine this driver cannot play on.
//!
//! The driver holds no hardware here: every claim was given back when the
//! bring-up found nothing to play through. It stays registered as
//! driver.hda0 only to answer `OP_OUTPUT_STATUS` with the reason, so that the
//! audio server and through it the player and Settings can say in plain
//! words why there is no sound. Every other request is answered `E_NODEV`.

use alloc::vec;

use nonos_libc::mk_ipc_recv;

use crate::controller::verdict_name::name;
use crate::protocol::{
    decode_request, refused, OutputStatus, E_INVAL, E_NODEV, E_OK, HDR_LEN, MAX_PCM_CHUNK,
    OP_HEALTHCHECK, OP_OUTPUT_STATUS, OUTPUT_STATUS_PAYLOAD_LEN, RESP_HDR_LEN, STATUS_LEN,
};
use crate::server::{error, handlers};
use crate::setup::Line;

const RX_LEN: usize = HDR_LEN + MAX_PCM_CHUNK;
const TX_LEN: usize = RESP_HDR_LEN + STATUS_LEN + OUTPUT_STATUS_PAYLOAD_LEN;
const RECV_TIMEOUT_MS: u64 = 1000;

pub fn run_status(status: OutputStatus) -> ! {
    Line::new("[HDA] no playable output: ")
        .s(name(status.verdict))
        .s(", verdict ")
        .dec(status.verdict)
        .s("; serving status only")
        .emit();
    let mut rx = vec![0u8; RX_LEN];
    let mut tx = vec![0u8; TX_LEN];
    loop {
        let n = mk_ipc_recv(0, rx.as_mut_ptr(), RX_LEN, RECV_TIMEOUT_MS);
        if !nonos_libc::recv_ready(n) {
            continue;
        }
        let msg = &rx[..n as usize];
        let Some(req) = decode_request(msg) else {
            error::reply_with_status(&mut tx, &refused(msg), E_INVAL);
            continue;
        };
        match req.op {
            OP_HEALTHCHECK => error::reply_with_status(&mut tx, &req, E_OK),
            OP_OUTPUT_STATUS => handlers::output_status::handle(&status, &req, &mut tx),
            _ => error::reply_with_status(&mut tx, &req, E_NODEV),
        }
    }
}
