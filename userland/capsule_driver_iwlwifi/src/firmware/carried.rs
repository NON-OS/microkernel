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
//! Which adapters this driver runs: those whose gen3 platform selects an
//! image the tree carries. Every other Intel Wi-Fi function is announced and
//! left unclaimed, so it never gets an interrupt line or DMA from this
//! capsule. The AX200, AX201, 9260, 9560, 8265 and 7265 have no gen3
//! platform; the discrete AX210 (TY) and Meteor Lake (MA) have one whose
//! firmware is not committed.

use super::blob::gen3_blob;
use super::gen3::select::{transport, Image};

/// The images a platform's MAC and RF may select (`gen3::select::select`):
/// TY is 0x2725 and MA is 0x2729 and 0x7E40 there; every other platform it
/// has is an SO.
fn images(pci_device: u16) -> &'static [Image] {
    match pci_device {
        0x2725 => &[Image::TyGf],
        0x2729 | 0x7E40 => &[Image::MaGf],
        _ if transport(pci_device).is_some() => &[Image::SoGf, Image::SoHr],
        _ => &[],
    }
}

/// Whether this driver can run the adapter with PCI device id `pci_device`.
pub fn runs(pci_device: u16) -> bool {
    images(pci_device).iter().any(|&image| gen3_blob(image).is_some())
}
