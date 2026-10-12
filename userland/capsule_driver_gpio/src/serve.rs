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

//! The capsule's one loop: a request served when one arrives within
//! `POLL_MS`, then every controller's pending pins taken apart. An `_AEI`
//! pin's event goes to capsule_acpi at once (a level pin masked until its
//! method has run); a device's pin waits in the queue for its holder; a
//! storming pin is masked and said once. The controller's shared line is
//! not bound yet: the kernel programs a low IRQ edge-high where these
//! controllers ask for level-low, so status is read on this cadence.

use crate::aei::aei_signal;
use crate::bank::Bank;
use crate::queue::Queue;
use crate::requests::answer;
use crate::say::say;
use alloc::vec;
use alloc::vec::Vec;
use nonos_gpio::{Controller, Delivery};
use nonos_libc::{mk_ipc_recv_from, mk_ipc_reply, mk_uptime_ms, recv_ready};

const POLL_MS: u64 = 5;
const OWN_INBOX: u64 = 0;

pub fn run(mut banks: Vec<Bank>) -> ! {
    let mut queue = Queue::default();
    let mut rx = vec![0u8; 64];
    loop {
        let mut sender = 0u32;
        let n = mk_ipc_recv_from(OWN_INBOX, rx.as_mut_ptr(), rx.len(), POLL_MS, &mut sender);
        if recv_ready(n) && n > 0 && sender != 0 {
            let reply = answer(&mut banks, &mut queue, &rx[..n as usize], sender);
            let _ = mk_ipc_reply(sender, reply.as_ptr(), reply.len());
        }
        let now = mk_uptime_ms().max(0) as u64;
        for b in banks.iter_mut() {
            for d in b.router.interrupt(&mut b.ctl, now) {
                deliver(b, &mut queue, d);
            }
        }
    }
}

fn deliver(b: &mut Bank, queue: &mut Queue, d: Delivery) {
    match d {
        Delivery::Acpi { pin } => {
            let level = b.level_aei.contains(&pin);
            if level {
                let _ = b.ctl.mask(pin, true);
            }
            let status = aei_signal(b.record.device_id, pin);
            if level {
                let _ = b.ctl.mask(pin, false);
            }
            if status != 0 {
                say(b"aei method did not run", b.uid, pin, &status);
            }
        }
        Delivery::Holder { device, pin } => queue.push(device, b.uid, pin),
        Delivery::Stormed { pin, owner } => say(b"pin masked for storming", b.uid, pin, &owner),
    }
}
