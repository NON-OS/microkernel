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

//! What each command does to the controller's slots (xHCI 1.2 section
//! 4.6): Enable Slot hands out the lowest free slot, as QEMU's does, and
//! Disable Slot frees one. An Address Device takes EP0's ring from the
//! input context, unless its device never finishes SET_ADDRESS. A No Op
//! names no slot and does nothing (section 4.6.2). The endpoint commands
//! are `hc_recovery`'s.

use super::hc::{Hc, SLOTS};
use crate::trb::Trb;

const SUCCESS: u8 = 1;
const TRB_ERROR: u8 = 5;
const NO_SLOTS: u8 = 9;
const SLOT_NOT_ENABLED: u8 = 11;
const ENABLE_SLOT: u32 = 9;
const DISABLE_SLOT: u32 = 10;
const ADDRESS_DEVICE: u32 = 11;
const NOOP: u32 = 23;
/// Reset Endpoint, Stop Endpoint and Set TR Dequeue Pointer.
const RECOVERY: std::ops::RangeInclusive<u32> = 14..=16;

impl Hc {
    /// The completion code and slot id of `trb`, or `None` when it never
    /// completes.
    pub(super) fn execute(&mut self, trb: &Trb) -> Option<(u8, u8)> {
        let slot = trb.slot_id();
        if trb.get_type() == NOOP {
            return Some((SUCCESS, 0));
        }
        if trb.get_type() == ENABLE_SLOT {
            let Some(s) = (1..=SLOTS).find(|&s| !self.enabled[s as usize]) else {
                return Some((NO_SLOTS, 0));
            };
            self.enabled[s as usize] = true;
            return Some((SUCCESS, s));
        }
        if slot == 0 || slot > SLOTS {
            return Some((TRB_ERROR, slot));
        }
        if !self.enabled[slot as usize] {
            return Some((SLOT_NOT_ENABLED, slot));
        }
        match trb.get_type() {
            DISABLE_SLOT => self.enabled[slot as usize] = false,
            ADDRESS_DEVICE if self.stall_address == Some(slot) => {
                self.stall_address = None;
                return None;
            }
            ty if RECOVERY.contains(&ty) => return Some((self.recover(trb), slot)),
            ADDRESS_DEVICE => {
                let ep0 = self.input_dw(trb, 2, 2) as u64 | (self.input_dw(trb, 2, 3) as u64) << 32;
                self.ep0[slot as usize] = (ep0 & !0xF, ep0 & 1 != 0);
            }
            ty => self.took(trb, ty),
        }
        Some((SUCCESS, slot))
    }
}
