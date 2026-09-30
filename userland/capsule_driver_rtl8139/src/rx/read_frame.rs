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

use super::advance::advance;
use super::copy_ring::copy_ring;
use super::ring_u16::ring_u16;
use crate::constants::regs::RX_STATUS_OK;
use crate::constants::MAX_ETHERNET_FRAME;
use crate::init::restart_rx;
use crate::setup::Driver;

/// The length the part shows while a frame is still being written (early RX).
const RX_STILL_ARRIVING: usize = 0xFFF0;
/// Longest frame plus CRC the part hands up with ROK set (Linux 8139too's
/// MAX_ETH_FRAME_SIZE + 4); a longer length is a corrupt header.
const RX_MAX_RAW: usize = 1792 + 4;
/// Shortest raw length a real header carries.
const RX_MIN_RAW: usize = 8;

pub(super) fn read_frame(
    driver: &mut Driver,
    out: &mut [u8],
) -> Result<Option<usize>, &'static str> {
    compiler_fence(Ordering::Acquire);
    let base = driver.rx_user_va;
    let off = driver.rx_offset;
    let status = ring_u16(base, off);
    let raw_len = ring_u16(base, off + 2) as usize;
    if raw_len == RX_STILL_ARRIVING {
        return Ok(None);
    }
    /*
     * Returning here without moving on read the same header forever: one bad
     * header ended receive until the capsule restarted. A header no good frame
     * carries means the position is lost, so receive starts over.
     */
    if (status & RX_STATUS_OK) == 0 || raw_len < RX_MIN_RAW || raw_len > RX_MAX_RAW {
        restart_rx(driver)?;
        return Err("rtl8139 rx ring restarted");
    }
    let frame_len = raw_len - 4;
    // A good frame the caller cannot take (a tagged full-size one): skip it.
    if frame_len > MAX_ETHERNET_FRAME || frame_len > out.len() {
        advance(driver, raw_len)?;
        return Err("rtl8139 rx frame too large");
    }
    copy_ring(base, off + 4, out, frame_len);
    advance(driver, raw_len)?;
    Ok(Some(frame_len))
}
