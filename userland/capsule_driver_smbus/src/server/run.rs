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

//! The driver's loop: each request waited for in the kernel and answered.

use nonos_libc::{mk_ipc_recv_from, mk_ipc_reply, recv_ready};

use super::answer::{answer, Info};
use crate::setup::Driver;
use crate::wire::{REPLY_MAX, REQUEST_MAX};

const SERVICE_INBOX: u64 = 0;

pub fn run(mut d: Driver) -> ! {
    let info = Info { vendor: d.found.vendor, device: d.found.device, io_ports: d.io_ports };
    let (mut rx, mut tx) = ([0u8; REQUEST_MAX], [0u8; REPLY_MAX]);
    loop {
        let mut sender = 0u32;
        let n = mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), 0, &mut sender);
        if !recv_ready(n) || sender == 0 {
            continue;
        }
        let msg = &rx[..(n as usize).min(rx.len())];
        let len = answer(&mut d.host, &info, msg, &mut tx);
        // A lost reply leaves the caller to its own timeout.
        let _ = mk_ipc_reply(sender, tx.as_ptr(), len);
    }
}
