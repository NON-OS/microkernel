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

//! The NNET frame service net.core and net.l2 speak, over the adapter:
//! nonos_usbnet's request decoding and answers, this driver's receive loop.
//! Between requests the channel is serviced, so a receive's completion is
//! never held back waiting for the stack.

use alloc::vec;

use nonos_libc::{mk_ipc_recv_from, mk_ipc_reply};
use nonos_usbnet::nic::ETH_FRAME_MAX;
use nonos_usbnet::nnet::{answer, decode, status, Request, Stats, DATA_AT, E_INVAL, HDR_LEN};
use nonos_usbnet::nnet::{STATION_DRAWN, STATION_HOST_REFUSED, STATION_HOST_UNANSWERED};
use nonos_usbnet::run::own_pid;
use nonos_usbnet::Nic;

use crate::nic::Net;
use crate::NAME;

use crate::probe::Verdict;

const SERVICE_INBOX: u64 = 0;
// The codes probe.rs gives are the frame service's.
const _: () = assert!(Verdict::Drawn.station() == STATION_DRAWN);
const _: () = assert!(Verdict::Unanswered.station() == STATION_HOST_UNANSWERED);
const _: () = assert!(Verdict::Refused.station() == STATION_HOST_REFUSED);
/// How long a receive waits for a request before the channel is looked at.
const WAIT_MS: u64 = 1;

/// Serve until the host breaks the channel.
pub fn run(net: &mut Net) {
    let mut rx = vec![0u8; HDR_LEN + ETH_FRAME_MAX + 64];
    let mut tx = vec![0u8; DATA_AT + 4 + ETH_FRAME_MAX + 64];
    let me = own_pid(core::str::from_utf8(NAME).unwrap_or(""));
    // Settings reads where the address came from in OP_STATS's word 9.
    let mut stats = Stats { station: net.station, ..Stats::default() };
    loop {
        let mut from = 0u32;
        let n = mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), WAIT_MS, &mut from);
        // Sender 0 is the kernel's own delivery; what this capsule sent
        // itself comes back as a reply no one took, and is dropped.
        if n > 0 && from != 0 && from != me {
            let n = (n as usize).min(rx.len());
            let len = match decode(&rx[..n]) {
                Some(req) => answer(Some(net), &mut stats, &req, &rx[HDR_LEN..n], &mut tx),
                None => {
                    let none = Request { op: 0, flags: 0, request_id: 0, payload_len: 0 };
                    status(&mut tx, &none, E_INVAL)
                }
            };
            let _ = mk_ipc_reply(from, tx.as_ptr(), len);
        }
        stats.ticked(net.tick());
        if net.link.dead {
            return;
        }
    }
}
