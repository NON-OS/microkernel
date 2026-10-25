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

//! A doorbell on a bulk DCI runs that ring's Normal TRBs. An OUT TRB's
//! bytes are what the device took (`Bulk::out`); an IN TRB gets the next of
//! `Bulk::answer`'s bytes, a Short Packet with the residue when they run out
//! inside it, else Success.

use nonos_libc::dma_host;

use super::hc::Hc;
use super::hc_input::trb_at;
use crate::constants::{LINK_TC, TRB_TYPE_LINK, TRB_TYPE_NORMAL};
use crate::event_ring::events::{transfer, CC_SHORT_PACKET, CC_SUCCESS};
use crate::trb::Trb;

impl Hc {
    pub fn run_bulk(&mut self, slot: u8, dci: u8) {
        let Some(&(mut ring, mut cycle)) = self.bulk.rings.get(&(slot, dci)) else { return };
        while let Some(host) = dma_host(ring) {
            let trb = trb_at(host);
            if trb.get_cycle() != cycle {
                break;
            }
            if trb.get_type() == TRB_TYPE_LINK {
                (ring, cycle) = (trb.get_pointer() & !0xF, cycle ^ (trb.d3 & LINK_TC != 0));
                continue;
            }
            if trb.get_type() == TRB_TYPE_NORMAL {
                let left = self.move_bytes(&trb, dci & 1 == 1);
                let code = if left == 0 { CC_SUCCESS } else { CC_SHORT_PACKET };
                self.producer.post(transfer(ring, code, left, slot, dci));
            }
            ring += 16;
        }
        self.bulk.rings.insert((slot, dci), (ring, cycle));
    }

    /// Move one Normal TRB's bytes; the residue.
    fn move_bytes(&mut self, trb: &Trb, dir_in: bool) -> u32 {
        let len = (trb.d2 & 0x1_FFFF) as usize;
        let Some(buf) = dma_host(trb.get_pointer()).map(|b| b as *mut u8) else {
            return len as u32;
        };
        if !dir_in {
            // SAFETY: the TRB names a live grant the driver wrote `len` bytes to.
            self.bulk.out.push(unsafe { std::slice::from_raw_parts(buf, len) }.to_vec());
            return 0;
        }
        let n = len.min(self.bulk.answer.len());
        let bytes: Vec<u8> = self.bulk.answer.drain(..n).collect();
        // SAFETY: the TRB names a live grant of at least `len` bytes.
        unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), buf, n) };
        (len - n) as u32
    }
}
