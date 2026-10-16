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

//! One TRB of an IN data stage: the device's bytes written where the TRB
//! points, and a Short Packet event when they end inside it with ISP set.

use nonos_libc::dma_host;

use super::hc::Hc;
use crate::constants::TRB_ISP;
use crate::event_ring::events::{transfer, CC_SHORT_PACKET};
use crate::trb::Trb;

impl Hc {
    /// Write what is left of `data` into the buffer `trb` at `at` names;
    /// true when it ran short.
    pub(super) fn fill(&mut self, at: u64, trb: &Trb, data: &mut Vec<u8>, slot: u8) -> bool {
        let len = (trb.d2 & 0x1_FFFF) as usize;
        let n = len.min(data.len());
        if let Some(host) = dma_host(trb.get_pointer()) {
            // SAFETY: the TRB names a live grant of at least `len` bytes.
            unsafe { std::ptr::copy_nonoverlapping(data.as_ptr(), host as *mut u8, n) };
        }
        data.drain(..n);
        if n == len {
            return false;
        }
        if trb.d3 & TRB_ISP != 0 {
            self.producer.post(transfer(at, CC_SHORT_PACKET, (len - n) as u32, slot, 1));
        }
        true
    }
}
