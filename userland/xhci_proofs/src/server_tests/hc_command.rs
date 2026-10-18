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

//! The command ring (xHCI 1.2 section 4.6): each TRB with the ring's cycle
//! run and answered in order. A command that never finishes leaves the
//! ring stuck with CRR set; CRCR.CA gives it up with Command Aborted, then
//! Command Ring Stopped names the TRB after it (section 4.6.1.2).

use nonos_devmodel::FakeBar;
use nonos_libc::dma_host;

use super::hc::Hc;
use super::hc_input::{trb_at, Ran};
use crate::conformance::model::{wrote64, CAP_LEN};
use crate::constants::{CRCR_LO, LINK_TC, TRB_TYPE_LINK};
use crate::event_ring::events::command;

const CRCR: usize = CAP_LEN + CRCR_LO as usize;
const CA: u32 = 1 << 2;
const CRR: u32 = 1 << 3;
const ABORTED: u8 = 25;
const RING_STOPPED: u8 = 24;

impl Hc {
    pub fn run_commands(&mut self, bar: &FakeBar) {
        if self.ring == 0 {
            let crcr = wrote64(bar, CRCR);
            (self.ring, self.cycle) = (crcr & !0x3F, crcr & 1 != 0);
        }
        while self.stuck.is_none() {
            let Some(host) = dma_host(self.ring) else { return };
            let trb = trb_at(host);
            if trb.get_cycle() != self.cycle {
                return;
            }
            if trb.get_type() == TRB_TYPE_LINK {
                self.ring = trb.get_pointer() & !0xF;
                self.cycle ^= trb.d3 & LINK_TC != 0;
                continue;
            }
            self.ran.push(Ran { trb, input: self.input_of(&trb) });
            let Some((code, slot)) = self.execute(&trb) else {
                self.stuck = Some((self.ring, trb.slot_id()));
                bar.present32(CRCR, CRR);
                return;
            };
            self.producer.post(command(self.ring, code, slot));
            self.ring += 16;
        }
    }

    pub fn abort(&mut self, bar: &FakeBar) {
        if bar.wrote32(CRCR) & CA == 0 {
            return;
        }
        bar.present32(CRCR, 0);
        let Some((at, slot)) = self.stuck.take() else { return };
        self.producer.post(command(at, ABORTED, slot));
        self.ring = at + 16;
        self.producer.post(command(self.ring, RING_STOPPED, 0));
    }
}
