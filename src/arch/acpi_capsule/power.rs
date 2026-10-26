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

//! What the clock and the battery syscall ask of ACPI, on every target.

/// From the clock's paced work: the kernel's poll of the fixed power button.
pub(crate) fn acpi_power_button_tick(ticks: u64) {
    #[cfg(target_arch = "x86_64")]
    super::super::x86_64::acpi::power_button::poll(ticks);
    #[cfg(not(target_arch = "x86_64"))]
    let _ = ticks;
}

/// Whether the ACPI tables declare a battery, None where they say nothing or
/// there are none: the kernel's own scan, beside the capsule's report.
pub(crate) fn acpi_tables_declare_battery() -> Option<bool> {
    #[cfg(target_arch = "x86_64")]
    return super::super::x86_64::acpi::parser::with_data(|d| d.power_devices.battery);
    #[cfg(not(target_arch = "x86_64"))]
    None
}
