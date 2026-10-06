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

use nonos_libc::mk_time_millis;

use crate::server::handlers::dns::resolve_a;
use crate::server::parse_req::{parse, HDR_LEN, IPC_BUF_MAX};
use crate::server::runner::{dispatch, receive, refuse};

// How often to re-check whether a better network interface has come up. The WiFi
// link is down at boot and only associates once the user connects, so the stack
// starts on whatever is up then (often nothing, or a cabled NIC) and must switch
// when the WiFi link appears.
const REEVAL_INTERVAL_MS: i64 = 1000;

pub fn run() -> ! {
    let mut rx = vec![0u8; HDR_LEN + IPC_BUF_MAX];
    let mut tx = vec![0u8; HDR_LEN + IPC_BUF_MAX];
    let mut last_reeval: i64 = 0;
    // Least time between device polls while the stack is quiet.
    //
    // `pump` runs a full smoltcp poll: every socket, every queue, the device.
    // It used to run on every turn of this loop, so the poll rate was whatever
    // the receive wait happened to be, and a single message a second held the
    // loop attentive long enough to poll about ninety times a second forever.
    // The wait tier was never the cost; this was. Under traffic the loop is
    // attentive anyway and polls at its own rate, so this floor only applies
    // to a stack with nothing to do.
    const IDLE_POLL_MS: i64 = 20;
    let mut last_poll: i64 = 0;

    loop {
        let now = mk_time_millis();
        // Poll when something is happening, a DNS lookup waits for its answer,
        // or the idle floor has passed.
        let waiting = resolve_a::waiting();
        if receive::attentive() || waiting || now.wrapping_sub(last_poll) >= IDLE_POLL_MS {
            crate::iface::poll::pump();
            last_poll = now;
        }
        // Lookups are answered here, after a poll, never pumped to their end
        // inside dispatch, where they held every other client for up to 3 s.
        if waiting {
            resolve_a::settle(&mut tx);
        }
        // Every call these make waits at most a few tens of milliseconds; a
        // join autojoin starts runs on in the driver and is watched, not waited
        // for.
        if now.wrapping_sub(last_reeval) >= REEVAL_INTERVAL_MS {
            crate::setup::reevaluate();
            crate::autojoin::tick(now);
            last_reeval = now;
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
