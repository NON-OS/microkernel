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


//! Receive, serve, reply; commit what is owed when it is due, and close
//! the files of clients that ended.

use alloc::vec;

use nonos_libc::{mk_ipc_recv_from, mk_ipc_reply, mk_uptime_ms};
use nonos_user_files::limits::DEFAULT_QUOTA;
use nonos_user_files::wire::MESSAGE_MAX;
use nonos_user_files::{serve, Broker, Host};

use crate::desk::Desk;
use crate::volume::Volume;

const SERVICE_PORT: u64 = 4424;
/// The longest a receive waits: an owed commit is never later than this
/// past its due time.
const WAIT_MS: u64 = 100;
/// How often the clients holding files are looked for among the living.
const REAP_MS: u64 = 2_000;

pub fn run() -> ! {
    let desk = Desk;
    let mut broker = Broker::new(Volume, DEFAULT_QUOTA);
    let mut msg = vec![0u8; MESSAGE_MAX];
    let mut reaped = 0u64;
    loop {
        let mut sender = 0u32;
        let got = mk_ipc_recv_from(SERVICE_PORT, msg.as_mut_ptr(), msg.len(), WAIT_MS, &mut sender);
        if got > 0 && sender != 0 {
            let reply = serve(&mut broker, &desk, sender, &msg[..got as usize]);
            let _ = mk_ipc_reply(sender, reply.as_ptr(), reply.len());
        }
        // A commit that fails stays owed, and is tried again at the next turn.
        let _ = broker.tick(desk.mono_ns());
        let now = u64::try_from(mk_uptime_ms()).unwrap_or(0);
        if now.saturating_sub(reaped) >= REAP_MS {
            broker.reap(&desk);
            reaped = now;
        }
    }
}
