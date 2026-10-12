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

//! What a slot may take on, and how much one interrupt read may ask. A
//! Bluetooth controller has an interrupt-IN endpoint (HCI events) beside a
//! bulk pair (ACL data) on one interface, so a slot takes interrupt
//! endpoints and one bulk pair together; each configure adds only its own
//! endpoints and only raises Context Entries (contexts/slot_copy.rs), so
//! neither drops the other. Pure, for xhci_proofs.

use super::resources::{SlotResources, MAX_INTERRUPT_ENDPOINTS};
use crate::protocol::INTERRUPT_IN_MAX;

/// wMaxPacketSize bits 10:0; bits 12:11 are high-bandwidth transactions.
const MAX_PACKET_MASK: u16 = 0x07FF;

impl SlotResources {
    /// Whether one more interrupt-IN endpoint fits, bulk pipes or not.
    pub fn takes_interrupt(&self) -> bool {
        self.interrupt.len() < MAX_INTERRUPT_ENDPOINTS
    }

    /// Whether the bulk pair fits: one a slot, interrupt endpoints or not.
    pub fn takes_bulk(&self) -> bool {
        self.bulk.is_none()
    }
}

/// The most one read on an interrupt-IN endpoint of `max_packet` may ask:
/// one packet, never past INTERRUPT_IN_MAX. A device's packet is never
/// longer than its descriptor says, so this is never babble.
pub fn interrupt_limit(max_packet: u16) -> u16 {
    (max_packet & MAX_PACKET_MASK).min(INTERRUPT_IN_MAX as u16)
}
