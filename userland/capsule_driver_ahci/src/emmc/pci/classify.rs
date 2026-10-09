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

//! The kind of host a PCI function is.

use super::ids::{PCI_CLASS_SYSTEM, PCI_SUBCLASS_SDHCI, PCI_VENDOR_INTEL};
use super::{INTEL_EMMC, INTEL_NOT_EMMC};

/// How discovery treats one PCI function.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    /// A known Intel eMMC host: served first.
    IntelEmmc,
    /// Another SDHCI host: served only if its slot is an embedded one.
    Generic,
}

/// The kind of host a PCI function is, or None when it is not an SDHCI
/// host or is one that must not be served (an SD reader or SDIO host).
pub fn classify(vendor: u16, device: u16, class: u8, subclass: u8, progif: u8) -> Option<Kind> {
    if class != PCI_CLASS_SYSTEM || subclass != PCI_SUBCLASS_SDHCI || progif > 0x02 {
        return None;
    }
    if vendor == PCI_VENDOR_INTEL {
        if INTEL_EMMC.contains(&device) {
            return Some(Kind::IntelEmmc);
        }
        if INTEL_NOT_EMMC.contains(&device) {
            return None;
        }
    }
    Some(Kind::Generic)
}
