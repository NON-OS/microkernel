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

//! AMD-Vi. Every unit is first taken back from firmware: a unit left
//! enabled translates every device's DMA through tables firmware built for
//! its own drivers, so the NVMe drive, the SATA controller and USB all fail
//! to reach the memory the kernel hands them; Linux disables such a unit
//! before anything else. A kernel built with `nonos-iommu-amdvi` then drives
//! the units (AMD IOMMU spec 48882): one shared device table, a command
//! buffer and an event log per unit, and capsule domains behind the same
//! broker calls as VT-d. Without the feature DMA on an AMD machine is
//! unrestricted and the posture line says so.

//! What AMD-Vi bring-up says on the console, and the call boot makes.

use super::release::release_from_firmware;
use crate::arch::x86_64::iommu::handover::amd_stop;
use crate::sys::serial;
#[cfg(feature = "nonos-iommu-amdvi")]
use crate::sys::serial::Line;

/// In a kernel built with `nonos-iommu-amdvi`, take every unit back from
/// firmware and put it in service with the kernel's tables. Without it, a
/// unit firmware left running keeps running (iommu::handover), but under the
/// development opt-in.
pub fn init() {
    let dev = crate::memory::iommu::dev_unconfined::holds();
    if amd_stop(cfg!(feature = "nonos-iommu-amdvi"), dev) {
        release_from_firmware();
    }
    #[cfg(feature = "nonos-iommu-amdvi")]
    match super::bringup::bring_up() {
        Ok(assigned) => {
            let mut line = Line::new();
            line.str(b"[AMD-VI] IOMMU translation enabled, units=");
            line.dec(super::units::units().len() as u64);
            line.str(b" reserved=").dec(assigned as u64).end();
            serial::println(b"[AMD-VI] every other requester id blocked until claimed");
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
