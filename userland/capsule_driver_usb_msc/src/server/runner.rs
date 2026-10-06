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

use nonos_libc::{mk_getpid, mk_ipc_recv_from, Deadline};

use crate::protocol::{parse, refused, BLK_HEADER_LEN, BLK_MAX_BYTES, E_INVAL, HDR_LEN, STATUS_LEN};
use crate::scan::{Medium, Scanner, RESCAN_MS};
use crate::server::dispatch::dispatch;
use crate::server::respond;
use crate::state::State;

const SERVICE_INBOX: u64 = 0;
/// While the device is being looked for, requests are answered between
/// passes: `E_AGAIN` for the block surface, as usual for the rest.
const SCAN_POLL_MS: u64 = 5;
/// With no device after the window, how long a receive waits before the
/// ports are looked at again.
const ABSENT_POLL_MS: u64 = 250;

pub fn run() -> ! {
    let mut rx = vec![0u8; HDR_LEN + BLK_HEADER_LEN + BLK_MAX_BYTES];
    let mut tx = vec![0u8; HDR_LEN + STATUS_LEN + BLK_MAX_BYTES];
    let mut state = State::new();
    let mut scanner = Scanner::new();
    let mut medium = Medium::Scanning;
    /*
     * A reply to the kernel's inbox that finds no waiter comes back here;
     * anything this capsule sent itself is dropped, never answered.
     */
    let self_pid = mk_getpid();
    let mut rescan = Deadline::after_ms(RESCAN_MS);
    loop {
        let scanning = matches!(medium, Medium::Scanning);
        let wait = match medium {
            Medium::Scanning => SCAN_POLL_MS,
            Medium::Absent => ABSENT_POLL_MS,
            Medium::Ready(_) => 0,
        };
        let mut sender = 0u32;
        let n = mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), wait, &mut sender);
        if n > 0 && sender != self_pid {
            match parse(&rx[..n as usize]) {
                Some((req, body)) => {
                    dispatch(&mut state, &medium, &scanner.report, sender, req, body, &mut tx)
                }
                None => {
                    let _ = respond::status(sender, &refused(&rx[..n as usize]), E_INVAL, &mut tx);
                }
            }
        }
        if scanning {
            medium = scanner.step(&mut state);
        } else if matches!(medium, Medium::Absent) && rescan.expired() {
            medium = scanner.again(&mut state);
            rescan = Deadline::after_ms(RESCAN_MS);
        }
    }
}
