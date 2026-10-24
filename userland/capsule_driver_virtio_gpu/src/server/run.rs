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
//! Every frame from the inbox answered by the wire server and the reply
//! sent to the pid the kernel says sent it. The loop wakes at least every
//! WATCH_MS, a frame or not, so a silent owner is seen (`Server::watch`):
//! its screen is blanked from zeroed primaries, the host's copies included.
//! Each turn reads the host's display events first, and an output that
//! changed (turned off or back, or a new size wanted) is told to the
//! compositor at once with OUTPUT_NOTICE rather than on its next poll.
//! Each turn also settles the control queue (`recover`): a stall the last
//! turn's commands hit is recovered in place, and a device left Down is
//! told to the compositor as LEAVING.

use nonos_libc::{mk_ipc_recv_from, mk_ipc_reply, mk_uptime_ms, recv_ready};

use nonos_gfx_pool::Pool;
use nonos_gfx_wire::notice::{LEAVING, OUTPUTS_CHANGED};

use super::gone::{leave, tell};
use super::recover::settle;
use super::turn::{turn, Turn};
use crate::driver::Driver;

const SERVICE_INBOX: u64 = 0;
const WATCH_MS: u64 = 500;

pub fn run(mut driver: Driver<Pool>) -> ! {
    // Requests are at most 256 bytes; replies the header, status and 252.
    let (mut rx, mut tx) = ([0u8; 256], [0u8; 280]);
    loop {
        if settle(&mut driver) {
            tell(driver.server.owner, LEAVING);
        }
        let server = &mut driver.server;
        if server.display.events() {
            tell(server.owner, OUTPUTS_CHANGED);
        }
        let mut sender = 0u32;
        let n = mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), WATCH_MS, &mut sender);
        let now = mk_uptime_ms().max(0) as u64;
        server.watch_said(now, |l| _ = nonos_libc::mk_debug(l.as_ptr(), l.len()));
        if !recv_ready(n) {
            continue;
        }
        match turn(sender, &rx[..n as usize], driver.device_id) {
            Turn::End => leave(server.owner),
            Turn::Drop => continue,
            Turn::Serve => {}
        }
        let len = server.handle(sender, &rx[..n as usize], &mut tx);
        server.heard(sender, now);
        if len > 0 {
            let _ = mk_ipc_reply(sender, tx.as_ptr(), len);
        }
    }
}
