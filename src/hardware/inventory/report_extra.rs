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

//! The devices a driver takes that the inventory has no family for, and
//! the ones nothing needs to drive, so the report names them rightly.

use super::family::HardwareFamily;
use super::report_text::Verdict;

/// QEMU's virtio entropy device, transitional and modern.
const VIRTIO_RNG: [u16; 2] = [0x1005, 0x1044];
/// The PNP ids of the PS/2 keyboard and mouse, and of the ACPI device.
const PNP_PS2: [u16; 2] = [0x0303, 0x0304];
const PNP_ACPI: u16 = 0x0C08;

/// The verdict for a device the family tables do not decide, or None.
pub(super) fn extra(
    pci: bool,
    pnp: bool,
    ids: (u16, u16),
    family: HardwareFamily,
) -> Option<Verdict<'static>> {
    let (vendor, device) = ids;
    if pci && vendor == 0x1AF4 && VIRTIO_RNG.contains(&device) {
        return Some(Verdict::Driver("driver_virtio_rng"));
    }
    if pnp && PNP_PS2.contains(&device) {
        return Some(Verdict::Driver("driver_ps2_input"));
    }
    if pnp && device == PNP_ACPI {
        return Some(Verdict::Driver("acpi"));
    }
    matches!(family, HardwareFamily::BridgePci | HardwareFamily::SystemPeripheral)
        .then_some(Verdict::NotNeeded)
}
