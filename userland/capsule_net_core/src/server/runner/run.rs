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
        // Poll when something is happening, or when the idle floor has passed.
        if receive::attentive() || now.wrapping_sub(last_poll) >= IDLE_POLL_MS {
            crate::iface::poll::pump();
            last_poll = now;
        }
        if now.wrapping_sub(last_reeval) >= REEVAL_INTERVAL_MS {
            crate::setup::reevaluate();
            last_reeval = now;
        }
        let mut sender_pid = 0u32;
        let n = receive::receive(&mut rx, &mut sender_pid);
        if n <= 0 || sender_pid == 0 {
            continue;
        }
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
