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

//! The endpoint commands on EP0 (xHCI 1.2 sections 4.6.8 to 4.6.10): Set
//! TR Dequeue moves the ring EP0 runs from, and an EP0 a STALL halted
//! (`hc_race`) answers Stop Endpoint and Set TR Dequeue with Context State
//! Error until a Reset Endpoint stops it.

use super::hc::Hc;
use super::hc_race::{ep_state, CONTEXT_STATE_ERROR, STOPPED};
use crate::trb::Trb;

const SUCCESS: u8 = 1;
const TRB_ERROR: u8 = 5;
const RESET_ENDPOINT: u32 = 14;
const SET_TR_DEQUEUE: u32 = 16;

impl Hc {
    /// The completion code of Reset Endpoint, Stop Endpoint or Set TR
    /// Dequeue Pointer `trb`.
    pub(super) fn recover(&mut self, trb: &Trb) -> u8 {
        let (slot, ep0) = (trb.slot_id(), trb.endpoint_id() == 1);
        if self.refuse_recovery {
            return TRB_ERROR;
        }
        let halted = self.race.halted.filter(|&(s, _)| s == slot && ep0);
        match (trb.get_type(), halted) {
            (RESET_ENDPOINT, Some((_, at))) => {
                self.race.halted = None;
                ep_state(at, STOPPED);
                SUCCESS
            }
            (RESET_ENDPOINT, None) if ep0 => match self.race.halt_at.take() {
                Some(at) => self.stall_after(slot, at),
                None => SUCCESS,
            },
            (_, Some(_)) => CONTEXT_STATE_ERROR,
            (SET_TR_DEQUEUE, None) if ep0 => {
                let to = trb.get_pointer();
                self.ep0[slot as usize] = (to & !0xF, to & 1 != 0);
                SUCCESS
            }
            _ => SUCCESS,
        }
    }
}
