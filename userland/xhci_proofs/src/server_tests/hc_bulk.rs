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

//! A slot's bulk pipes (xHCI 1.2 section 4.11.2.3): Configure Endpoint
//! hands the controller each added endpoint's ring, which a doorbell
//! naming that DCI runs (`hc_bulk_run`).

use std::collections::HashMap;

use nonos_devmodel::FakeBar;

use super::hc::Hc;
use super::hc_device::STS;
use crate::constants::{TRB_TYPE_CONFIGURE_ENDPOINT_CMD, USBSTS_HCE};
use crate::trb::Trb;

#[derive(Default)]
pub struct Bulk {
    /// Each (slot, DCI)'s ring dequeue pointer and cycle state.
    pub rings: HashMap<(u8, u8), (u64, bool)>,
    /// Every OUT TRB's bytes, in order.
    pub out: Vec<Vec<u8>>,
    /// What the device sends on its IN endpoints.
    pub answer: Vec<u8>,
}

impl Hc {
    /// A slot's doorbell: a bulk ring Configure Endpoint named, else EP0.
    pub(super) fn doorbell(&mut self, bar: &FakeBar, slot: u8, dci: u32) {
        if self.bulk.rings.contains_key(&(slot, dci as u8)) {
            return self.run_bulk(slot, dci as u8);
        }
        if self.race.fail_at_ep0 {
            bar.present32(STS, bar.wrote32(STS) | USBSTS_HCE);
        }
        self.run_ep0(slot);
    }

    /// A command `execute` takes as done: a Configure Endpoint's rings.
    pub(super) fn took(&mut self, trb: &Trb, ty: u32) {
        if ty != TRB_TYPE_CONFIGURE_ENDPOINT_CMD {
            return;
        }
        let add = self.input_dw(trb, 0, 1);
        for dci in (2..32u8).filter(|d| add & 1 << d != 0) {
            let at = dci as u64 + 1;
            // Bulk OUT (2) and Bulk IN (6) only: the interrupt pipes stay unanswered.
            if !matches!(self.input_dw(trb, at, 1) >> 3 & 7, 2 | 6) {
                continue;
            }
            let ring = self.input_dw(trb, at, 2) as u64 | (self.input_dw(trb, at, 3) as u64) << 32;
            self.bulk.rings.insert((trb.slot_id(), dci), (ring & !0xF, ring & 1 != 0));
        }
    }
}
