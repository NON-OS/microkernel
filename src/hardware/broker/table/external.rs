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

//! The PCIe ports firmware calls external-facing and the buses behind each,
//! with DEVICE_FLAG_EXTERNAL set on every function they reach, those added
//! after the boot scan included. Kept beside the table rather than in the
//! ACPI broker: the table and the IOMMU that reads the flag are not x86's.
//! A flag once set stays set: a port is never less external than it was.

extern crate alloc;

use alloc::vec::Vec;
use spin::Mutex;

use crate::drivers::pci::types::PciAddress;
use crate::hardware::broker::external_range::{behind, ExternalRange};

// Taken before TABLE and never while holding it.
static RANGES: Mutex<Vec<ExternalRange>> = Mutex::new(Vec::new());

fn bdf(a: &PciAddress) -> (u8, u8, u8) {
    (a.bus, a.device, a.function)
}

/// The range a function sits in, if any: for the IOMMU's domain choice.
pub fn external_port_of(a: &PciAddress) -> Option<ExternalRange> {
    RANGES.lock().iter().find(|r| behind(r, a.segment, bdf(a))).copied()
}

crate::arch::acpi_only! {
    mod late;
    pub(in crate::hardware::broker) use late::mark;
    pub(super) use late::flag_for;
}
