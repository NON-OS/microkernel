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

//! What a slot keeps that the controller may still reach: the contexts
//! and EP0 ring of an Address Device that failed, and the request buffers,
//! rings and report buffers a failed controller may have been given. They
//! stay until Disable Slot completes and `take_resources` hands them back
//! to be dropped, or the whole table goes once the controller was reset
//! (`revive`) or released (teardown). A request buffer only a transfer not
//! brought back points at stays with its ring instead (`TransferRing::keep`).

use super::super::SlotResources;
use super::types::SlotTable;
use crate::dma::DmaRegion;

impl SlotTable {
    /// Keep the resources of `resources.slot_id`, whose Address Device
    /// failed after the controller was given them, unaddressed: no request
    /// reaches them (`resources_mut`), and the slot is not addressed again
    /// before it is disabled (`has_resources`).
    pub fn hold(&mut self, resources: SlotResources) {
        self.resources.push(resources);
    }

    /// Whether `slot_id` holds resources, addressed or held.
    pub fn has_resources(&self, slot_id: u8) -> bool {
        self.resources.iter().any(|r| r.slot_id == slot_id)
    }

    /// Keep `buf` with `slot_id` until the slot is dropped. With no such
    /// slot it is never freed: leaked, it is memory, not a frame the
    /// controller may write after the kernel reused it.
    pub fn quarantine(&mut self, slot_id: u8, buf: DmaRegion) {
        match self.resources.iter_mut().find(|r| r.slot_id == slot_id) {
            Some(r) => r.quarantine.push(buf),
            None => core::mem::forget(buf),
        }
    }
}
