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

use nonos_libc::{mk_ipc_recv, mk_irq_wait};

use crate::protocol::{
    decode_request, E_INVAL, HDR_LEN, OP_CONTROLLER_STATUS, OP_GET_STATE, OP_HEALTHCHECK,
    OP_POLL_MOUSE,
};
use crate::server::context::Context;
use crate::server::error::{reply_decode_failed, reply_with_status};
use crate::server::handlers;
use crate::server::pump::pump;
use crate::server::reinit::reinit_mouse;
use crate::server::runner::consts::{IRQ_WAIT_MS, POLL_IDLE_MS, TX_LEN};
use crate::server::upkeep::sync_leds;
use crate::setup::Driver;

pub fn run(driver: Driver) -> ! {
    let mut rx = vec![0u8; HDR_LEN];
    let mut tx = vec![0u8; TX_LEN];
    let mut ctx = Context::new(driver);
    let mut prev_buttons = 0u8;
    let mut wait_seq = 0u64;
    loop {
        pump(&mut ctx, &mut prev_buttons);
        reinit_mouse(&mut ctx);
        sync_leds(&mut ctx);
        let n = mk_ipc_recv(0, rx.as_mut_ptr(), HDR_LEN, POLL_IDLE_MS);
        if !nonos_libc::recv_ready(n) {
            let _ = mk_irq_wait(0, wait_seq, IRQ_WAIT_MS, &mut wait_seq);
            continue;
        }
        let len = (n as usize).min(HDR_LEN);
        let req = match decode_request(&rx[..len]) {
            Some(r) => r,
            None => {
                reply_decode_failed(&mut tx, E_INVAL);
                continue;
            }
        };
        if req.payload_len != 0 {
            reply_with_status(&mut tx, &req, E_INVAL);
            continue;
        }
        match req.op {
            OP_HEALTHCHECK => handlers::health::handle(&req, &mut tx),
            OP_POLL_MOUSE => handlers::mouse::handle(&mut ctx, &req, &mut tx),
            OP_GET_STATE => handlers::state::handle(&mut ctx, &req, &mut tx),
            OP_CONTROLLER_STATUS => handlers::controller_status::handle(&mut ctx, &req, &mut tx),
            _ => reply_with_status(&mut tx, &req, E_INVAL),
        }
    }
}
