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

//! Native display engines, each only where the inventory saw its function:
//! the driver takes over at firmware's mode, so the screen stays lit.

#[cfg(feature = "nonos-capsule-driver-intel-display")]
pub(super) fn spawn() {
    use crate::hardware::intel_display_capsule as c;
    use crate::hardware::inventory::{present, HardwareFamily};
    if !present(HardwareFamily::DisplayNativeIntel) {
        crate::sys::boot_log::ok("DRIVER-INTEL-DISPLAY", "no controller present, not spawned");
        return;
    }
    super::boot::capsule(
        "DRIVER-INTEL-DISPLAY",
        "driver_intel_display",
        c::spawn_driver_intel_display_capsule,
        c::shared_state,
    );
}
#[cfg(not(feature = "nonos-capsule-driver-intel-display"))]
pub(super) fn spawn() {}
