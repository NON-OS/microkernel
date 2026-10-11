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

//! The serve loop. Only the compositor may send here (the kernel holds the
//! endpoint to it), and of its senders the first to take the primary
//! surface is the only one that may draw.

use nonos_libc::mk_ipc_recv_from;

use super::dispatch::dispatch;
use super::respond::status;
use crate::protocol::{parse, refused, E_INVAL};
use crate::setup::Driver;

const RX_LEN: usize = 128;
const TX_LEN: usize = 64;

pub fn run(driver: &mut Driver) -> ! {
    let (mut rx, mut tx) = ([0u8; RX_LEN], [0u8; TX_LEN]);
    loop {
        let mut sender = 0u32;
        let n = mk_ipc_recv_from(0, rx.as_mut_ptr(), RX_LEN, 0, &mut sender);
        if !nonos_libc::recv_ready(n) || sender == 0 {
            continue;
        }
        let msg = &rx[..(n as usize).min(RX_LEN)];
        let Some((req, body)) = parse(msg) else {
            status(sender, &refused(msg), E_INVAL, &mut tx);
            continue;
        };
        let errno = dispatch(driver, sender, &req, body, &mut tx);
        if let Some(errno) = errno {
            status(sender, &req, errno, &mut tx);
        }
    }
}
