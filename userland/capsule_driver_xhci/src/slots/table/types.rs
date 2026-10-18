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
use super::super::SlotResources;
use alloc::vec::Vec;

pub(super) const XHCI_SLOT_TABLE_LEN: usize = 256;

pub struct SlotTable {
    pub(super) allocated: [bool; XHCI_SLOT_TABLE_LEN],
    pub(super) addressed: [bool; XHCI_SLOT_TABLE_LEN],
    /// The pid of the client that enabled each slot; 0 for none, or for the
    /// kernel's own requests.
    pub(super) owner: [u32; XHCI_SLOT_TABLE_LEN],
    pub(super) resources: Vec<SlotResources>,
    pub(super) count: u16,
}
