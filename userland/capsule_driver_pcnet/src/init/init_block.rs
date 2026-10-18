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

//! The SWSTYLE 2 init block, 28 bytes the part reads once on CSR0.INIT:
//! mode, ring lengths as log2, the station address, the multicast filter,
//! then the two ring addresses.

use core::sync::atomic::{fence, Ordering};

use crate::constants::layout::{INIT_BLOCK_BYTES, INIT_BLOCK_OFF};
use crate::constants::queue::{RX_LOG2, TX_LOG2};
use crate::constants::MAC_LEN;

/// Mode 0: normal operation, broadcast received, no promiscuous mode.
const MODE_NORMAL: u16 = 0;

pub fn encode(mac: &[u8; MAC_LEN], rx_ring: u32, tx_ring: u32) -> [u8; INIT_BLOCK_BYTES] {
    let mut b = [0u8; INIT_BLOCK_BYTES];
    b[0..2].copy_from_slice(&MODE_NORMAL.to_le_bytes());
    b[2] = RX_LOG2 << 4;
    b[3] = TX_LOG2 << 4;
    b[4..10].copy_from_slice(mac);
    // Bytes 12..20 are the logical address filter, left zero: no multicast
    // group is joined, as e1000's cleared table array.
    b[20..24].copy_from_slice(&rx_ring.to_le_bytes());
    b[24..28].copy_from_slice(&tx_ring.to_le_bytes());
    b
}

/// Write the block into the control page, ahead of the INIT that reads it.
pub fn store(control_va: u64, block: &[u8; INIT_BLOCK_BYTES]) {
    let dst = (control_va as usize + INIT_BLOCK_OFF) as *mut u8;
    // SAFETY: the control grant is a whole page and the block ends well
    // inside it (constants::layout asserts so); nothing else aliases it.
    unsafe { core::ptr::copy_nonoverlapping(block.as_ptr(), dst, block.len()) };
    fence(Ordering::Release);
}
