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

use crate::constants::dma::{TX_SLOT_BYTES, TX_SLOT_COUNT};
use crate::constants::regs::{REG_TXSTATUS0, TSD_ERTXTH_256};
use crate::constants::MIN_WIRE_FRAME;
use crate::setup::Driver;

/*
 * Hand one frame to the next slot. The caller has reclaimed and checked for
 * room, so the part is finished with this slot. The frame is answered once it
 * is queued: the part sends it when it can, and `reclaim` sees it done. The
 * part does not pad, so a short frame is zero-filled to the 60-byte minimum.
 */
pub fn send(driver: &mut Driver, frame: &[u8]) -> Result<(), &'static str> {
    let idx = driver.tx_cur % TX_SLOT_COUNT;
    let va = driver.tx_user_va + (idx * TX_SLOT_BYTES) as u64;
    let wire = frame.len().max(MIN_WIRE_FRAME);
    unsafe {
        let dst = va as *mut u8;
        core::ptr::copy_nonoverlapping(frame.as_ptr(), dst, frame.len());
        core::ptr::write_bytes(dst.add(frame.len()), 0, wire - frame.len());
    }
    compiler_fence(Ordering::Release);
    let status_reg = REG_TXSTATUS0 + (idx as u16 * 4);
    driver.pio.w32(status_reg, wire as u32 | TSD_ERTXTH_256)?;
    driver.tx_cur = driver.tx_cur.wrapping_add(1);
    Ok(())
}
