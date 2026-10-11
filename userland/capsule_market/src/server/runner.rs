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

use nonos_libc::mk_ipc_recv_from;

use crate::bonds::Bonds;
use crate::clock::now_ms;
use crate::protocol::{
    decode_request, E_INVAL, E_MSGSIZE, HDR_LEN, OP_GET_APP, OP_GET_COMMERCE, OP_GET_RELEASE,
    OP_HEALTHCHECK, OP_INSTALL_READY, OP_LIST_APPS, OP_LOAD_INDEX, RX_BUF_LEN, TX_BUF_LEN,
};
use crate::server::error::{reply_decode_failed, reply_status};
use crate::server::handlers;
use crate::store::Store;
use crate::verify::Verifier;

pub fn run<V: Verifier>(store: &mut Store, verifier: &V) -> ! {
    let mut rx = vec![0u8; RX_BUF_LEN];
    let mut tx = vec![0u8; TX_BUF_LEN];
    let mut bonds = Bonds::default();
    loop {
        let mut sender = 0u32;
        let n = mk_ipc_recv_from(0, rx.as_mut_ptr(), RX_BUF_LEN, bonds.wait_ms(), &mut sender);
        bonds.tick(now_ms());
        // Settle's answer to a bond read is taken here and needs no reply.
        if n <= 0 || bonds.on_frame(sender, rx.get(..n as usize).unwrap_or(&[]), now_ms()) {
            continue;
        }
        let n = n as usize;
        let req = match decode_request(&rx[..n]) {
            Some(r) => r,
            None => {
                reply_decode_failed(&mut tx, E_INVAL);
                continue;
            }
        };
        let body_start = HDR_LEN;
        let body_end = body_start.saturating_add(req.payload_len as usize);
        if body_end > n {
            reply_status(&mut tx, &req, E_MSGSIZE);
            continue;
        }
        let body = &rx[body_start..body_end];
        match req.op {
            OP_HEALTHCHECK => handlers::health::handle(&req, &mut tx),
            OP_LOAD_INDEX => handlers::load_index::handle(store, verifier, body, &req, &mut tx),
            OP_LIST_APPS => handlers::list_apps::handle(store, &req, &mut tx),
            OP_GET_APP => handlers::get_app::handle(store, body, &req, &mut tx),
            OP_GET_RELEASE => handlers::get_release::handle(store, body, &req, &mut tx),
            OP_INSTALL_READY => {
                handlers::install_ready::handle(store, &mut bonds, body, &req, &mut tx);
                bonds.pump(now_ms());
            }
            OP_GET_COMMERCE => handlers::get_commerce::handle(store, body, &req, &mut tx),
            _ => reply_status(&mut tx, &req, E_INVAL),
        }
    }
}
