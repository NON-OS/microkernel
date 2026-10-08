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

//! What AMD-Vi bring-up says on the console, and the call boot makes.

use super::release::release_from_firmware;
use crate::sys::serial;
#[cfg(feature = "nonos-iommu-amdvi")]
use crate::sys::serial::Line;

/// Take every unit back from firmware, then, in a kernel built with
/// `nonos-iommu-amdvi`, put them in service with the kernel's tables.
pub fn init() {
    release_from_firmware();
    #[cfg(feature = "nonos-iommu-amdvi")]
    match super::bringup::bring_up() {
        Ok(assigned) => {
            let mut line = Line::new();
            line.str(b"[AMD-VI] IOMMU translation enabled, units=");
            line.dec(super::units::units().len() as u64);
            line.str(b" devices=").dec(assigned as u64).end();
            serial::println(b"[AMD-VI] enumerated devices identity mapped; others denied");
        }
        Err(e) => {
            let mut line = Line::new();
            line.str(b"[AMD-VI] IOMMU bring-up failed (").str(reason(e));
            line.str(b"); DMA is unrestricted").end();
        }
    }
    #[cfg(not(feature = "nonos-iommu-amdvi"))]
    serial::println(b"[AMD-VI] IOMMU driver not built in; DMA is unrestricted");
}

#[cfg(feature = "nonos-iommu-amdvi")]
fn reason(e: super::error::AmdViError) -> &'static [u8] {
    use super::error::AmdViError as E;
    match e {
        E::NotPresent => b"no unit in IVRS",
        E::RegistersUnmappable => b"register window not mappable",
        E::NoFrames => b"no contiguous 2 MiB for the device table",
        E::TableUnreachable => b"table outside the directmap",
        E::Timeout => b"unit did not complete a command",
        _ => b"unexpected error",
    }
}
