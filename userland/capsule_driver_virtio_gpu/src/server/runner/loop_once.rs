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

//! The server loop. It wakes at least every WATCH_MS, a message or not, so
//! the watch (G-73) sees a dead or silent owner with no request coming in.

use nonos_libc::{mk_ipc_recv_from, mk_pid_alive, mk_uptime_ms};

use super::dispatch::dispatch;
use crate::death::watch::watch;
use crate::driver::Driver;
use crate::protocol::{parse, refused, E_INVAL};
use crate::server::respond;

const SERVICE_INBOX: u64 = 0;
const WATCH_MS: u64 = 500;

pub fn loop_once(driver: Driver, rx: &mut [u8], tx: &mut [u8]) -> ! {
    loop {
        let mut sender_pid = 0u32;
        let n =
            mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), WATCH_MS, &mut sender_pid);
        let now = mk_uptime_ms().max(0) as u64;
        let owner = driver.owner.pid.get();
        watch(&driver, now, owner == 0 || mk_pid_alive(owner));
        if !nonos_libc::recv_ready(n) || sender_pid == 0 {
            continue;
        }
        driver.owner.heard(sender_pid, now);
        let Some((req, body)) = parse(&rx[..n as usize]) else {
            let _ = respond::status(sender_pid, &refused(&rx[..n as usize]), E_INVAL, tx);
            continue;
        };
        dispatch(&driver, sender_pid, req, body, now, tx);
    }
}
