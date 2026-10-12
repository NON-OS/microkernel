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

//! The serve loop: one request at a time, answered before the next.

use alloc::vec;

use nonos_libc::{mk_ipc_recv_from, recv_ready};

use crate::protocol::{REPLY_MAX, REQUEST_MAX};
use crate::scrub::scrub;
use crate::server::dispatch::serve;
use crate::server::recover::recover;
use crate::setup::Driver;

pub fn run(driver: &mut Driver) -> ! {
    let mut rx = vec![0u8; REQUEST_MAX];
    let mut tx = vec![0u8; REPLY_MAX];
    let mut kept = None;

    loop {
        let mut sender_pid: u32 = 0;
        let n = mk_ipc_recv_from(0, rx.as_mut_ptr(), rx.len(), 0, &mut sender_pid);
        if !recv_ready(n) || sender_pid == 0 {
            continue;
        }
        let Some(len) = usize::try_from(n).ok().filter(|&n| n <= rx.len()) else {
            continue;
        };
        let stalled = serve(sender_pid, driver, &rx[..len], &mut kept, &mut tx);
        // A frame handed in to send is delivered: none of it stays here.
        scrub(&mut rx[..len]);
        if stalled {
            recover(driver);
        }
    }
}
