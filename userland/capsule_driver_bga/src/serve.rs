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
use nonos_libc::{mk_ipc_recv_from, mk_ipc_reply, recv_ready};

use crate::display::Bga;

const SERVICE_INBOX: u64 = 0;

pub fn run(server: &mut Server<Bga, Pool>) -> ! {
    // Requests are at most 256 bytes; replies the header, status and 252.
    let (mut rx, mut tx) = ([0u8; 256], [0u8; 280]);
    loop {
        let mut sender = 0u32;
        let n = mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), 0, &mut sender);
        if !recv_ready(n) || sender == 0 {
            continue;
        }
        let len = server.handle(sender, &rx[..n as usize], &mut tx);
        if len > 0 {
            let _ = mk_ipc_reply(sender, tx.as_ptr(), len);
        }
    }
}
