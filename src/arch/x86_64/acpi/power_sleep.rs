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

use super::error::{AcpiError, AcpiResult};
use super::parser;
use super::power_prepared::{prepare_shutdown, shutdown_prepared};
use super::power_types::SleepState;

pub fn enter_sleep_state(state: SleepState) -> AcpiResult<()> {
    match state {
        SleepState::S0 => Ok(()),
        SleepState::S5 => shutdown(),
        // S1..S4 need _PTS, _WAK and a resume path; none exists.
        _ => Err(AcpiError::PowerStateNotSupported),
    }
}

/// Enter S5 (soft off).
///
/// SLP_TYPa/b come from the `\_S5` package found at init, or else from the
/// ACPI capsule's interpreter (`broker::acpi::s5_choice`). The register
/// sequence is ACPICA's (`hw::sleep`): clear the fixed and GPE status bits,
/// disable every GPE, write SLP_TYP to PM1a and PM1b control, then SLP_EN.
/// A hardware-reduced FADT uses SLEEP_CONTROL_REG instead.
///
/// `_PTS(5)` is the ACPI capsule's to run before it asks for this (F-111);
/// the kernel has no interpreter. Returns only when the machine still runs
/// afterwards. As ACPICA does for S4/S5, the enable write is repeated once
/// after a wait, for chipsets that miss the first.
pub fn shutdown() -> AcpiResult<()> {
    shutdown_prepared(&prepare_shutdown(parser::with_data)?)
}
