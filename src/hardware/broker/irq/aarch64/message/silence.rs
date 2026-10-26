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

//! Stopping the function from writing a bind's messages, before the
//! controller is told to drop them. Best effort: a function gone from
//! config space writes nothing either.

use crate::drivers::pci::config::ConfigSpace;
use crate::drivers::pci::msi::{disable_msix, mask_all_msix, mask_msix_vector, zero_msix_vector};
use crate::hardware::broker::pci_index::{msi_of, PciHandle};

use super::msi_program::clear;
use super::record::MessageBind;

/// Vector `i` masked and its entry zeroed, or the one MSI turned off.
pub(super) fn vector(b: &MessageBind, handle: Option<&PciHandle>, i: usize) {
    let Some(h) = handle else {
        return;
    };
    if !b.msix {
        if let Some(msi) = msi_of(h) {
            let _ = clear(&h.address, &msi);
        }
    } else if let Some(msix) = h.msix {
        let _ = mask_msix_vector(&msix, &h.bars, i as u16);
        let _ = zero_msix_vector(&msix, &h.bars, i as u16);
    }
}

/// MSI-X off for the whole function once no vector is left.
pub(super) fn function(b: &MessageBind, handle: Option<&PciHandle>) {
    if let Some((h, msix)) = handle.filter(|_| b.msix).and_then(|h| h.msix.map(|m| (h, m))) {
        let cfg = ConfigSpace::new(h.address);
        let _ = mask_all_msix(&cfg, &msix);
        let _ = disable_msix(&cfg, &msix);
    }
}
