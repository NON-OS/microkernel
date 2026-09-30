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

use core::sync::atomic::{compiler_fence, Ordering};

use crate::constants::queue::TX_DESC_COUNT;
use crate::constants::regs::{DESC_EOR, DESC_FS, DESC_LS, DESC_OWN, REG_TX_POLL, TX_POLL_NPQ};
use crate::constants::MIN_WIRE_FRAME;
use crate::queue::desc::{desc, desc_mut, Descriptor};
use crate::setup::Driver;

/// Whether the part still owns the next slot, so a frame has nowhere to go.
pub fn busy(driver: &Driver) -> bool {
    (unsafe { desc(driver.tx.desc_va, driver.tx.cur) }.opts1 & DESC_OWN) != 0
}

/*
 * Hand one frame to the part and answer once it is queued. The cursor moves
 * on the moment OWN goes over: the part walks the ring with a pointer of its
 * own and moves past the slot when it finishes, so a cursor held back on a
 * TX error or a slow completion (link still negotiating, PAUSE frames) was
 * one slot behind the part for good, and every later send saw "busy".
 * The caller checks `busy` first; OWN still set on the next slot is a full
 * ring.
 */
pub fn send(driver: &mut Driver, frame: &[u8]) {
    let idx = driver.tx.cur;
    let wire = frame.len().max(MIN_WIRE_FRAME);
    unsafe {
        let dst = driver.tx.buffer_va(idx) as *mut u8;
        core::ptr::copy_nonoverlapping(frame.as_ptr(), dst, frame.len());
        core::ptr::write_bytes(dst.add(frame.len()), 0, wire - frame.len());
    }
    compiler_fence(Ordering::Release);
    let eor = if idx == TX_DESC_COUNT - 1 { DESC_EOR } else { 0 };
    let addr = driver.tx.buffer_da(idx);
    let d = Descriptor {
        opts1: DESC_OWN | DESC_FS | DESC_LS | eor | wire as u32,
        opts2: 0,
        addr_lo: addr as u32,
        addr_hi: (addr >> 32) as u32,
    };
    unsafe {
        desc_mut(driver.tx.desc_va, idx, d);
        driver.regs.w8(REG_TX_POLL, TX_POLL_NPQ);
    }
    driver.tx.cur = (idx + 1) % TX_DESC_COUNT;
}
