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

use alloc::vec;

use nonos_libc::mk_uptime_ms;

use crate::server::handlers::dns::resolve_a;
use crate::server::parse_req::{parse, HDR_LEN, IPC_BUF_MAX};
use crate::server::runner::turn::{turn, Turn};
use crate::server::runner::{dispatch, receive, refuse};

pub fn run() -> ! {
    let mut rx = vec![0u8; HDR_LEN + IPC_BUF_MAX];
    let mut tx = vec![0u8; HDR_LEN + IPC_BUF_MAX];
    let (mut probed, mut polled) = (i64::MIN, i64::MIN);

    loop {
        let now = mk_uptime_ms();
        let waiting = resolve_a::waiting();
        match turn(now, probed, polled, receive::attentive() || waiting) {
            // Every call these make waits at most a few tens of milliseconds;
            // a join autojoin starts runs on in the driver and is watched, not
            // waited for. Re-checking for a better interface is how the stack
            // moves to the Wi-Fi link once the user connects.
            Turn::Probe => {
                crate::iface::relink::check();
                crate::setup::reevaluate();
                crate::autojoin::tick(now);
                probed = now;
            }
            Turn::Poll => {
                crate::iface::poll::pump();
                polled = now;
            }
            Turn::Neither => {}
        }
        // Lookups are answered here, after a poll, never pumped to their end
        // inside dispatch, where they held every other client for up to 3 s.
        if waiting {
            resolve_a::settle(&mut tx);
        }
        let mut sender_pid = 0u32;
        let n = receive::receive(&mut rx, &mut sender_pid);
        crate::server::reap::reap_if_due();
        if n <= 0 || sender_pid == 0 {
            continue;
        }
        // A caller with a lookup still waiting gave up on it; that call is owed
        // its reply before anything answers this one.
        resolve_a::settle_for(sender_pid, &mut tx);
        let raw = &rx[..n as usize];
        let (req, body) = match parse(raw) {
            Ok(parsed) => parsed,
            Err(errno) => {
                refuse::refuse(sender_pid, raw, errno, &mut tx);
                continue;
            }
        };
        dispatch::dispatch(sender_pid, &req, body, &mut tx);
    }
}
