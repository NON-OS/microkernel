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
//! The driver's loop: each request waited for in the kernel and answered,
//! and a holder the kernel says is gone let go between requests.

use alloc::vec;

use nonos_libc::{mk_ipc_recv_from, mk_ipc_reply, mk_pid_alive, recv_ready};

use crate::frame::decode;
use crate::ops::answer;
use crate::owner::Owner;
use crate::setup::Host;
use crate::wire::{CHUNK, HDR_LEN, REPLY_MAX};

const SERVICE_INBOX: u64 = 0;
const WAIT_MS: u64 = 1_000;

pub fn run(h: Host) -> ! {
    let (mut rx, mut tx) = (vec![0u8; HDR_LEN + CHUNK], vec![0u8; REPLY_MAX]);
    let mut owner = Owner::default();
    loop {
        let mut from = 0u32;
        let n = mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), WAIT_MS, &mut from);
        if owner.pid.is_some_and(|p| !mk_pid_alive(p)) {
            owner.close();
        }
        if !recv_ready(n) || from == 0 {
            continue;
        }
        if let Some((req, body)) = decode(&rx[..(n as usize).min(rx.len())]) {
            let len = answer(&h, &mut owner, from, &req, body, &mut tx);
            let _ = mk_ipc_reply(from, tx.as_ptr(), len);
        }
    }
}
