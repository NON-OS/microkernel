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

//! The power service, then the ACPI capsule, started before the drivers:
//! neither claims a PCI function, and the services that will ask about
//! batteries, lids and power find them already up. The power service is
//! the one owner of `power.off`, the shell's wiped power-off; the ACPI
//! capsule claims no such name and serves the button alone
//! (docs/build/feature-inventory.md).

pub(super) fn spawn() {
    spawn_power();
    spawn_acpi();
}

#[cfg(feature = "nonos-capsule-acpi")]
fn spawn_acpi() {
    use crate::userspace::capsule_acpi as c;
    super::boot::capsule("ACPI", "acpi", c::spawn_acpi_capsule, c::shared_state);
}

#[cfg(not(feature = "nonos-capsule-acpi"))]
fn spawn_acpi() {}

#[cfg(feature = "nonos-capsule-power")]
fn spawn_power() {
    use crate::userspace::capsule_power as c;
    super::boot::capsule("POWER", "power", c::spawn_power_capsule, c::shared_state);
}

#[cfg(not(feature = "nonos-capsule-power"))]
fn spawn_power() {}
