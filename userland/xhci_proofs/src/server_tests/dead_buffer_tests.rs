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

//! A control request whose wait ended because the controller reported a
//! Host Controller Error. HCE does not halt the controller, which may still
//! write the request's buffer (xHCI 1.2 section 5.4.2), so the buffer is
//! kept with its slot and no command goes to bring EP0 back; the revival
//! that resets the controller frees it.

use nonos_libc::{dma_host, start_log, take_log, Call};

use super::bench::Bench;
use super::hc_device::{lock, IDLE};
use super::hc_reset::Resets;
use crate::conformance::model::{CAP_LEN, DOORBELL};
use crate::constants::{USBSTS, USBSTS_HCE};
use crate::protocol::{E_IO, OP_CONTROL_TRANSFER, OP_ENABLE_SLOT};

const HIGH_SPEED: u32 = 3;

#[test]
fn a_request_buffer_outlives_a_failed_controller_until_its_reset() {
    let mut b = Bench::new(HIGH_SPEED);
    let slot = b.root_device();
    let op = b.mux.ctxs[0].driver.layout.op_base;
    b.mux.ctxs[0].driver.event_ring.set_op_base(op);
    let (hc, bar) = (b.hc.clone(), b.bar.clone());
    let model = lock(&hc);
    let (door, sts) = (DOORBELL + 4 * slot as usize, CAP_LEN + USBSTS as usize);
    let fail = std::thread::spawn(move || {
        while bar.wrote32(door) == IDLE {
            std::thread::yield_now();
        }
        bar.present32(sts, bar.wrote32(sts) | USBSTS_HCE);
    });
    start_log(|| 0);
    let get_status = [slot, 0, 0x80, 0, 0, 0, 0, 0, 2, 0];
    assert_eq!(b.call(OP_CONTROL_TRANSFER, &get_status).status, E_IO);
    let freed = take_log().iter().filter(|l| matches!(l.call, Call::DmaUnmap(_))).count();
    assert_eq!(b.bar.wrote32(DOORBELL), IDLE, "no command for a failed controller");
    drop(model);
    fail.join().expect("the controller failed");
    assert_eq!(freed, 0, "nothing freed while the controller may still write it");
    let d = &mut b.mux.ctxs[0].driver;
    let res = d.slots.resources_mut(slot, d.layout.max_slots).expect("the slot");
    let kept = res.quarantine.first().map(|buf| buf.phys()).expect("the buffer kept");
    assert!(dma_host(kept).is_some());
    lock(&b.hc).resets = Resets::Followed;
    assert_eq!(b.call(OP_ENABLE_SLOT, &[]).status, 0, "served once revived");
    assert_eq!(b.mux.ctxs[0].driver.generation, 1);
    assert!(dma_host(kept).is_none(), "the revival freed it, the controller reset");
}
