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

//! Starting the part: the rings' addresses, the setup frame through the
//! transmit process alone, and once the part has taken it, the receive
//! process too. Each step is checked against a deadline.

use nonos_libc::Deadline;

use crate::constants::csr::{CSR3, CSR4, CSR5, CSR6, CSR6_SR, CSR6_ST};
use crate::constants::queue::SETUP_FRAME_LEN;
use crate::constants::MAC_LEN;
use crate::setup::Driver;

use super::reset::running;
use super::setup_frame;

/// A real part takes microseconds for either.
const SETUP_WAIT_MS: u64 = 20;
const START_WAIT_MS: u64 = 20;

pub fn run(d: &mut Driver, mac: &[u8; MAC_LEN]) -> Result<(), &'static str> {
    d.regs.set_csr(CSR3, d.rx.ring_device_addr as u32);
    d.regs.set_csr(CSR4, d.tx.ring_device_addr as u32);
    let frame = setup_frame::encode(mac);
    // SAFETY: slot `tail` (0, the ring is clear) of the transmit pool is
    // BUFFER_LEN bytes, longer than the frame, and the part does not own it.
    let va = d.tx.buffer_va(d.tx.tail) as *mut u8;
    let slot = unsafe { core::slice::from_raw_parts_mut(va, SETUP_FRAME_LEN) };
    slot.copy_from_slice(&frame);
    let idx = d.tx.post_setup(SETUP_FRAME_LEN as u16);
    d.regs.set_csr(CSR6, CSR6_ST);
    d.regs.tx_poll();
    wait(SETUP_WAIT_MS, || d.tx.done(idx)).map_err(|_| "setup frame not taken")?;
    d.tx.reclaim();
    d.regs.set_csr(CSR6, CSR6_ST | CSR6_SR);
    wait(START_WAIT_MS, || running(d.regs.csr(CSR5))).map_err(|_| "part did not start")
}

fn wait(ms: u64, mut done: impl FnMut() -> bool) -> Result<(), ()> {
    let deadline = Deadline::after_ms(ms);
    while !done() {
        if deadline.expired() {
            return Err(());
        }
        core::hint::spin_loop();
    }
    Ok(())
}
