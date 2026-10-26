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

//! The ACPI capsule's exit, a power-button press while it restarts (F-107)
//! and the critical watches' clocks (F-108), callable on every target so
//! teardown and init's loop stay one path. Only x86_64 runs the capsule;
//! elsewhere there is nothing held. `gate` says which targets are ACPI
//! platforms (`acpi_only!`).

mod gate;
mod power;

pub(crate) use gate::acpi_only;
pub(crate) use power::{acpi_power_button_tick, acpi_tables_declare_battery};

/// From process teardown, for every pid: the ACPI capsule's holds given
/// back and its events disarmed, the Global Lock with them if it held it.
pub(crate) fn acpi_capsule_exit(pid: u32) {
    #[cfg(target_arch = "x86_64")]
    if let Some(lock) = crate::hardware::broker::acpi::leave::on_exit(pid) {
        super::x86_64::acpi::leave::events(lock);
    }
    #[cfg(not(target_arch = "x86_64"))]
    let _ = pid;
}

/// From init's loop: the wiped power-off for a press read in the gap, or
/// for a critical watch whose grace ran out with no reading since.
pub(crate) fn acpi_from_init() {
    #[cfg(target_arch = "x86_64")]
    crate::hardware::broker::acpi::leave::serve_from_init();
}

/// From init, right after the ACPI capsule's spawn (L55): its drivers wait
/// for the capsule's first publish pass, bounded and said once. Elsewhere
/// there is no capsule, and nothing waits.
pub(crate) fn acpi_first_pass_wait(spawned: bool) {
    #[cfg(target_arch = "x86_64")]
    super::x86_64::acpi::first_pass_wait::wait(spawned);
    #[cfg(not(target_arch = "x86_64"))]
    let _ = spawned;
}
