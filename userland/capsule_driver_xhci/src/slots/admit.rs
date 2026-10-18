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

//! What a slot may take on. A Bluetooth controller has an interrupt-IN endpoint (HCI events) beside a
//! bulk pair (ACL data) on one interface, so a slot takes interrupt
//! endpoints and one bulk pair together; each configure adds only its own
//! endpoints and only raises Context Entries (contexts/slot_copy.rs), so
//! neither drops the other. Pure, for xhci_proofs.

use super::interrupt::MAX_INTERRUPT_ENDPOINTS;
use super::resources::SlotResources;

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
