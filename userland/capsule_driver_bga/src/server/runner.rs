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

use nonos_libc::{mk_ipc_recv_from, recv_ready};

use super::{dispatch, respond};
use crate::protocol::{parse, refused, E_INVAL, REQUEST_MAX};
use crate::setup::Driver;

const SERVICE_INBOX: u64 = 0;
/// A reply is a header, a status and at most the primary surface's 32 bytes.
const REPLY_MAX: usize = 64;

/// Waits in the kernel for each request: an idle desktop costs no CPU.
pub fn run(mut driver: Driver) -> ! {
    let mut rx = [0u8; REQUEST_MAX];
    let mut tx = [0u8; REPLY_MAX];
    loop {
        let mut sender = 0u32;
        let n = mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), 0, &mut sender);
        if !recv_ready(n) || sender == 0 {
            continue;
        }
        let frame = &rx[..(n as usize).min(rx.len())];
        match parse(frame) {
            Some((req, body)) => dispatch::dispatch(&mut driver, sender, &req, body, &mut tx),
            // The caller is blocked until a reply comes, so a refusal is one.
            None => respond::status(sender, &refused(frame), E_INVAL, &mut tx),
        }
    }
}
