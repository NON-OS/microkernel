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

//! The server loop: a request, then one step of the search for the device
//! or of its check, each in its own window (`serve`).

use alloc::vec;

use nonos_libc::{mk_getpid, mk_ipc_recv_from};

use super::serve::Server;
use crate::protocol::{BLK_HEADER_LEN, BLK_MAX_BYTES, HDR_LEN, STATUS_LEN};

const SERVICE_INBOX: u64 = 0;

pub fn run() -> ! {
    let mut rx = vec![0u8; HDR_LEN + BLK_HEADER_LEN + BLK_MAX_BYTES];
    let mut tx = vec![0u8; HDR_LEN + STATUS_LEN + BLK_MAX_BYTES];
    let mut server = Server::new();
    /*
     * A reply to the kernel's inbox that finds no waiter comes back here;
     * anything this capsule sent itself is dropped, never answered.
     */
    let self_pid = mk_getpid();
    loop {
        let wait = server.scanner.wait_ms();
        let mut sender = 0u32;
        let n = mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), wait, &mut sender);
        if n > 0 && sender != self_pid {
            server.serve(sender, &rx[..n as usize], &mut tx);
        }
        server.step();
    }
}
