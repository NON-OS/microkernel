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
//! server, the reply sent to the pid the kernel says sent it.

use nonos_gfx_pool::Pool;
use nonos_gfx_serve::Server;
use nonos_libc::{mk_ipc_recv_from, mk_ipc_reply, mk_uptime_ms, recv_ready};

use crate::display::Svga;
use crate::turn::{turn, Turn};

const SERVICE_INBOX: u64 = 0;
/// Wake at least this often, so an owner's silence is seen (`Server::watch`).
const WATCH_MS: u64 = 500;

pub fn run(server: &mut Server<Svga, Pool>, device_id: u64) -> ! {
    // Requests are at most 256 bytes; replies the header, status and 252.
    let (mut rx, mut tx) = ([0u8; 256], [0u8; 280]);
    loop {
        let mut sender = 0u32;
        let n = mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), WATCH_MS, &mut sender);
        let now = mk_uptime_ms().max(0) as u64;
        server.watch_said(now, |l| _ = nonos_libc::mk_debug(l.as_ptr(), l.len()));
        if !recv_ready(n) {
            continue;
        }
        match turn(sender, &rx[..n as usize], device_id) {
            Turn::End => crate::gone::leave(server.owner),
            Turn::Drop => continue,
            Turn::Serve => {}
        }
        let len = server.handle(sender, &rx[..n as usize], &mut tx);
        server.heard(sender, now);
        if len > 0 {
            let _ = mk_ipc_reply(sender, tx.as_ptr(), len);
        }
    }
}
