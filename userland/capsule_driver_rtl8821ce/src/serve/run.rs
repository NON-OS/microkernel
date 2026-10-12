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

//! The serving loop: one request answered per pass (answer.rs), then the
//! link's clock and a step of the background scan (after.rs).

use nonos_libc::{mk_ipc_recv_from, mk_ipc_reply};
use nonos_wifi_core::netif::{wire, MAX_RESPONSE};

use crate::setup::Mapped;
use crate::status;

use super::after::after_pass;
use super::answer::answer;
use super::connect::{self, Session};
use super::radio::{build_radio, Radio};
use super::scanner::Scanner;
use super::tuning::{IDLE_TIMEOUT_MS, SERVICE_INBOX};
use super::Stage;

/// Serve forever. Takes the mapped chip if bring-up got that far, and the stage it
/// reached. This never exits and never hangs: even a failed bring-up keeps the
/// service registered and answers the panel with the stage, which is the only
/// diagnostic a machine without a serial port has.
pub fn run(mapped: Option<Mapped>, stage: Stage) -> ! {
    let (mut radio, stage) = build_radio(mapped, stage);
    /*
     * A warm reboot does not power the card down, so a BSSID and the linked
     * network type a join of the last boot set can still be in the MAC. The
     * radio starts unbound from any BSS, as rtw88 sets RTW_NET_NO_LINK when
     * its interface comes up, so the scan hears every network.
     */
    if let Radio::Up(up) = &radio {
        connect::set_media_no_link(&up.regs);
    }
    let mut session: Option<Session> = None;
    let mut scanner = Scanner::new();
    status::line(b"[rtl8821ce] serving net_core\n");

    let mut rx = [0u8; wire::HDR_LEN + wire::MAX_FRAME];
    let mut tx = [0u8; MAX_RESPONSE];
    loop {
        // Reply to the sender's pid, not a shared reply endpoint: net_core and the
        // panel both call this driver, and a fixed endpoint routes answers by a
        // FIFO that crosses under concurrent callers, so net_core's link-status
        // reply could land in the panel's inbox.
        let mut sender_pid = 0u32;
        let n = mk_ipc_recv_from(
            SERVICE_INBOX,
            rx.as_mut_ptr(),
            rx.len(),
            IDLE_TIMEOUT_MS,
            &mut sender_pid,
        );
        if n > 0 && sender_pid != 0 {
            let req = &rx[..n as usize];
            let len = answer(req, sender_pid, &mut radio, &mut session, &scanner, stage, &mut tx);
            let _ = mk_ipc_reply(sender_pid, tx.as_ptr(), len);
        }
        after_pass(&mut radio, &mut session, &mut scanner);
    }
}
