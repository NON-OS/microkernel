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

//! The receive loop: every frame from the inbox answered by the wire
//! server, the reply sent to the pid the kernel says sent it. A frame too
//! short for the wire's header is F 12's brightness bytes, or a monitor
//! control beside them (at most eight; the wire's every request is at
//! least twenty), answered natively.

use nonos_gfx_pool::Pool;
use nonos_gfx_serve::Server;
use nonos_gfx_wire::header::HDR_LEN;
use nonos_libc::{mk_ipc_recv_from, mk_ipc_reply, recv_ready};

use crate::setup::Engine;
use crate::turn::{turn, Turn};
use nonos_intel_display::display::{brightness_answer, Rights, BRIGHTNESS_REPLY, OP_VCP};

const SERVICE_INBOX: u64 = 0;

pub fn run(server: &mut Server<Engine, Pool>, device_id: u64) -> ! {
    // Requests are at most 256 bytes; replies the header, status and 252,
    // or a brightness reply of at most 512.
    let (mut rx, mut tx) = ([0u8; 256], [0u8; BRIGHTNESS_REPLY]);
    loop {
        let mut sender = 0u32;
        let n = mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), 0, &mut sender);
        if !recv_ready(n) {
            continue;
        }
        match turn(sender, &rx[..n as usize], device_id) {
            Turn::End => crate::gone::leave(),
            Turn::Drop => continue,
            Turn::Serve => {}
        }
        let req = &rx[..n as usize];
        let len = if req.len() < HDR_LEN {
            let reads = req.first() == Some(&OP_VCP);
            let read_controls = reads && crate::senders::may_read_controls(sender);
            let may = Rights { set: crate::senders::may_set(sender), read_controls };
            let d = &mut server.display;
            brightness_answer(d, d.count() as u16, req, may, &mut tx)
        } else {
            server.handle(sender, req, &mut tx[..280])
        };
        if len > 0 {
            let _ = mk_ipc_reply(sender, tx.as_ptr(), len);
        }
    }
}
