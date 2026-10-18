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

//! What the controller may still reach is not freed before Disable Slot:
//! the contexts and EP0 ring of an Address Device that failed, and the
//! buffer of a control request whose endpoint could not be brought back.
//! Freed at once, a TRB the controller still ran would DMA into a frame
//! the kernel had already handed to someone else.

use nonos_libc::{dma_host, start_log, take_log, Call};

use super::bench::Bench;
use super::hc_device::lock;
use crate::protocol::{E_INVAL, E_IO, OP_ADDRESS_DEVICE, OP_CONTROL_TRANSFER, OP_DISABLE_SLOT};

const HIGH_SPEED: u32 = 3;

fn unmaps() -> usize {
    take_log().iter().filter(|l| matches!(l.call, Call::DmaUnmap(_))).count()
}

#[test]
fn a_failed_address_device_keeps_its_contexts_until_disable_slot() {
    let mut b = Bench::new(HIGH_SPEED);
    let slot = b.enable_slot();
    lock(&b.hc).stall_address = Some(slot);
    start_log(|| 0);
    assert_eq!(b.call(OP_ADDRESS_DEVICE, &[slot, 1]).status, E_IO);
    assert_eq!(unmaps(), 0, "the slot is still enabled: nothing it was given is freed");
    let entry = b.mux.ctxs[0].driver.dcbaa.phys() + 8 * slot as u64;
    // SAFETY: `dma_host` names the live DCBAA, entry `slot` inside it.
    let named = || unsafe { (dma_host(entry).expect("DCBAA") as *const u64).read_volatile() };
    assert_ne!(named(), 0, "the DCBAA still names its output context");
    assert_eq!(b.call(OP_ADDRESS_DEVICE, &[slot, 1]).status, E_INVAL, "not addressed again");
    start_log(|| 0);
    assert_eq!(b.call(OP_DISABLE_SLOT, &[slot]).status, 0);
    assert_eq!(unmaps(), 3, "output and input contexts and EP0 ring, once disabled");
    assert_eq!(named(), 0);
}

#[test]
fn a_request_buffer_stays_with_its_slot_when_recovery_fails() {
    let mut b = Bench::new(HIGH_SPEED);
    let slot = b.root_device();
    (lock(&b.hc).stall_in, lock(&b.hc).refuse_recovery) = (true, true);
    start_log(|| 0);
    let get_status = [slot, 0, 0x80, 0, 0, 0, 0, 0, 2, 0];
    // EP0 never came back, so the STALL is not answered as a clear pipe.
    assert_eq!(b.call(OP_CONTROL_TRANSFER, &get_status).status, E_IO);
    assert_eq!(unmaps(), 0, "EP0 never came back, so its TRB still names the buffer");
    start_log(|| 0);
    assert_eq!(b.call(OP_DISABLE_SLOT, &[slot]).status, 0);
    assert_eq!(unmaps(), 4, "contexts, EP0 ring and the kept buffer");
}
