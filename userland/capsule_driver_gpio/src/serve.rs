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

//! The capsule's one loop: a request served when one arrives, then every
//! controller whose interrupt fired (or that has no bound interrupt) has
//! its pending pins taken apart (`deliver`) and its line acknowledged.
//! With every line bound the loop sleeps on the interrupts and only
//! glances for requests; with any unbound it reads on `POLL_MS`.

use crate::bank::Bank;
use crate::deliver::deliver;
use crate::gone::kernel_message;
use crate::irq::{ack, fired};
use crate::irq_wait::{all_bound, wait};
use crate::queue::Queue;
use crate::requests::answer;
use alloc::vec;
use alloc::vec::Vec;
use nonos_libc::{mk_ipc_recv_from, mk_ipc_reply, mk_uptime_ms, recv_ready};

const POLL_MS: u64 = 5;
/// A request's look while interrupts are bound: the wait does the sleeping.
const GLANCE_MS: u64 = 1;
const OWN_INBOX: u64 = 0;

pub fn run(mut banks: Vec<Bank>) -> ! {
    let mut queue = Queue::default();
    let mut rx = vec![0u8; 64];
    let mut last = 0u64;
    let recv_ms = if all_bound(&banks) { GLANCE_MS } else { POLL_MS };
    loop {
        let mut sender = 0u32;
        let n = mk_ipc_recv_from(OWN_INBOX, rx.as_mut_ptr(), rx.len(), recv_ms, &mut sender);
        if recv_ready(n) && n > 0 && sender == 0 {
            kernel_message(&mut banks, &rx[..n as usize]);
        } else if recv_ready(n) && n > 0 {
            let reply = answer(&mut banks, &mut queue, &rx[..n as usize], sender);
            let _ = mk_ipc_reply(sender, reply.as_ptr(), reply.len());
        }
        wait(&banks, &mut last);
        let now = mk_uptime_ms().max(0) as u64;
        for b in banks.iter_mut().filter_map(|b| fired(b).then_some(b)) {
            for d in b.router.interrupt(&mut b.ctl, now) {
                deliver(b, &mut queue, d);
            }
            ack(b);
        }
    }
}
