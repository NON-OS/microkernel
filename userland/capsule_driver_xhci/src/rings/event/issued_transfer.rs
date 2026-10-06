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
//! A transfer TRB the driver issued, named the way its Transfer Event names
//! it: the TRB's address, and the slot and endpoint whose ring holds it
//! (xHCI 1.2 section 6.4.2.1). The controller writes all three, so an event
//! that differs in any of them is not this TRB's completion, whatever its
//! completion code says, and is never taken as one.
use crate::constants::TRB_TYPE_TRANSFER_EVENT;
use crate::trb::Trb;
#[derive(Clone, Copy)]
pub struct IssuedTransfer {
    pub phys: u64,
    pub slot: u8,
    pub dci: u8,
}
impl IssuedTransfer {
    /// Whether `event` is this TRB's Transfer Event. Pointer bits 3:0 are
    /// reserved, so they are not compared.
    pub fn completed_by(&self, event: &Trb) -> bool {
        event.get_type() == TRB_TYPE_TRANSFER_EVENT
            && event.get_pointer() & !0xF == self.phys & !0xF
            && event.slot_id() == self.slot
            && event.endpoint_id() == self.dci
    }
}
