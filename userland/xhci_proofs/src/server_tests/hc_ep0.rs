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

//! A slot's default control pipe (xHCI 1.2 section 4.11.2.2). An IN data
//! stage TD gets the bytes `answer` gives for the setup packet; when they
//! run out inside a TRB with ISP set, a Short Packet event names that TRB
//! and the rest of the TD is skipped (section 4.10.1.1). The Status Stage
//! completes with Success.

use nonos_libc::dma_host;

use super::hc::Hc;
use super::hc_input::trb_at;
use crate::constants::{
    LINK_TC, TRB_TYPE_DATA_STAGE, TRB_TYPE_LINK, TRB_TYPE_NORMAL, TRB_TYPE_SETUP_STAGE,
    TRB_TYPE_STATUS_STAGE,
};
use crate::event_ring::events::{transfer, CC_STALL, CC_SUCCESS};

pub use super::hc_setup::Setup;

impl Hc {
    pub fn run_ep0(&mut self, slot: u8) {
        let (mut ring, mut cycle) = self.ep0[slot as usize];
        let (mut setup, mut data, mut short) = (Setup::default(), Vec::new(), false);
        while let Some(host) = dma_host(ring) {
            let trb = trb_at(host);
            if trb.get_cycle() != cycle {
                break;
            }
            match trb.get_type() {
                TRB_TYPE_LINK => {
                    (ring, cycle) = (trb.get_pointer() & !0xF, cycle ^ (trb.d3 & LINK_TC != 0));
                    continue;
                }
                TRB_TYPE_SETUP_STAGE => {
                    (setup, short) = (Setup::of(&trb), false);
                    data = (self.answer)(&setup);
                }
                TRB_TYPE_DATA_STAGE if self.race.nak_in && setup.request_type & 0x80 != 0 => break,
                TRB_TYPE_DATA_STAGE if self.stall_in && setup.request_type & 0x80 != 0 => {
                    // Halted on the data stage: the ring goes no further.
                    self.producer.post(transfer(ring, CC_STALL, 0, slot, 1));
                    break;
                }
                TRB_TYPE_DATA_STAGE | TRB_TYPE_NORMAL
                    if !short && setup.request_type & 0x80 != 0 =>
                {
                    short = self.fill(ring, &trb, &mut data, slot);
                }
                TRB_TYPE_STATUS_STAGE => self.producer.post(transfer(ring, CC_SUCCESS, 0, slot, 1)),
                _ => {}
            }
            ring += 16;
        }
        self.ep0[slot as usize] = (ring, cycle);
    }
}
