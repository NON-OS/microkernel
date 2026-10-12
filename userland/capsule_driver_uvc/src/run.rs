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
//! The driver's loop: requests answered, the stream pumped while it runs,
//! a camera looked for while there is none, and a camera gone from its
//! port given back (its stream stopped first).

use alloc::vec;

use nonos_libc::{mk_ipc_recv_from, mk_ipc_reply, Deadline};
use nonos_usbnet::run::own_pid;
use nonos_usbnet::say::say;
use nonos_usbnet::scan::Scanner;

use crate::bind::bind;
use crate::device::Dev;
use crate::gone::{give_back, gone};
use crate::isoch::REPLY_LEN;
use crate::ops::answer;
use crate::stream::pump;
use crate::wire::{decode, HDR_LEN, REPLY_MAX};

const TAG: &[u8] = b"uvc";
const SEARCH_MS: u64 = 1_000;
const SERVICE_INBOX: u64 = 0;

pub fn run() -> ! {
    let (mut rx, mut tx) = (vec![0u8; HDR_LEN + 64], vec![0u8; REPLY_MAX]);
    let mut batch = [0u8; REPLY_LEN];
    let (mut d, mut scanner, me) = (Dev::default(), Scanner::new(), own_pid("driver.uvc0"));
    let mut next = Deadline::after_ms(0);
    say(TAG, &[b"started, looking for a camera"]);
    loop {
        let wait = if d.stream.is_some() { 0 } else { SEARCH_MS };
        let mut from = 0u32;
        let n = mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), wait, &mut from);
        if n > 0 && from != 0 && from != me {
            let msg = &rx[..(n as usize).min(rx.len())];
            if let Some((req, body)) = decode(msg) {
                let len = answer(&mut d, from, &req, body, &mut tx);
                let _ = mk_ipc_reply(from, tx.as_ptr(), len);
            }
        }
        if d.stream.is_some() {
            pump(&mut d, &mut batch);
        }
        if !next.expired() {
            continue;
        }
        next = Deadline::after_ms(SEARCH_MS);
        if gone(&d) {
            give_back(&mut d, &mut scanner);
        }
        if d.bound.is_none() {
            d.bound = scanner.pass(TAG, bind);
        }
    }
}
