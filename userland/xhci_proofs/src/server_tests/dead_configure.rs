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

//! Configuring an endpoint as the controller fails, and the revival after.

use nonos_libc::{dma_host, start_log, take_log, Call};

use super::bench::Bench;
use super::hc_device::{lock, IDLE};
use super::hc_reset::Resets;
use crate::conformance::model::{CAP_LEN, DOORBELL};
use crate::constants::{USBSTS, USBSTS_HCE};
use crate::protocol::{E_IO, OP_ENABLE_SLOT};

const HIGH_SPEED: u32 = 3;

/// `body` sent for `op` on a device's slot, the controller failing once
/// it is asked to run a command: what was kept, by bus address, and how
/// much was freed.
pub fn configured_as_it_fails(op: u16, body: impl Fn(u8) -> Vec<u8>) -> (Bench, Vec<u64>, usize) {
    let mut b = Bench::new(HIGH_SPEED);
    let slot = b.root_device();
    let op_base = b.mux.ctxs[0].driver.layout.op_base;
    b.mux.ctxs[0].driver.event_ring.set_op_base(op_base);
    let (hc, bar) = (b.hc.clone(), b.bar.clone());
    let model = lock(&hc);
    let sts = CAP_LEN + USBSTS as usize;
    let fail = std::thread::spawn(move || {
        while bar.wrote32(DOORBELL) == IDLE {
            std::thread::yield_now();
        }
        bar.present32(sts, bar.wrote32(sts) | USBSTS_HCE);
    });
    start_log(|| 0);
    assert_eq!(b.call(op, &body(slot)).status, E_IO, "op {op:#x}");
    let freed = take_log().iter().filter(|l| matches!(l.call, Call::DmaUnmap(_))).count();
    drop(model);
    fail.join().expect("the controller failed");
    // The failed controller takes the Configure Endpoint, as silicon would
    // have while the request waited, before the revival resets it.
    while b.bar.wrote32(DOORBELL) != IDLE {
        std::thread::yield_now();
    }
    let d = &mut b.mux.ctxs[0].driver;
    let res = d.slots.resources_mut(slot, d.layout.max_slots).expect("the slot");
    let kept = res.quarantine.iter().map(|r| r.phys()).collect();
    (b, kept, freed)
}

pub fn freed_by_the_revival(mut b: Bench, kept: &[u64]) {
    assert!(kept.iter().all(|&at| dma_host(at).is_some()), "still the controller's");
    lock(&b.hc).resets = Resets::Followed;
    assert_eq!(b.call(OP_ENABLE_SLOT, &[]).status, 0, "served once revived");
    assert!(kept.iter().all(|&at| dma_host(at).is_none()), "freed once it was reset");
}
