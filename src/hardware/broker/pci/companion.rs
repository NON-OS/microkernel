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

//! Whether an EHCI on an Intel xHCI's own bus is held by anyone, for the
//! port routing rule (quirk_bits/xhci_route.rs): a claim at any stage counts,
//! one being made or torn down too, so no port moves out from under it.

use crate::drivers::pci::config::ConfigSpace;
use crate::drivers::pci::types::PciAddress;
use crate::hardware::broker::{claim, pci_index};

use super::quirk_bits::blocks;

pub(super) fn ehci_claimed(xhci: PciAddress) -> bool {
    let at = (xhci.segment, xhci.bus);
    pci_index::all().iter().any(|h| {
        let class = ConfigSpace::new(h.address).read32(0x08).map_or(0, |c| c >> 8);
        let claimed = claim::claimed(h.device_id);
        blocks(at, (h.address.segment, h.address.bus), class, claimed)
    })
}
