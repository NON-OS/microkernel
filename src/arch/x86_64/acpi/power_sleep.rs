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
use super::hw::port_bus::{busy_wait_us, PortBus};
use super::hw::fadt_decode::FadtInfo;
use super::hw::sleep::{enter_sleep, SleepError, SleepTypes};
use super::parser;
use super::aml::sleep_obj::SleepPackage;
use super::power_types::SleepState;
use crate::hardware::broker::acpi::s5_choice::choose;

pub fn enter_sleep_state(state: SleepState) -> AcpiResult<()> {
    match state {
        SleepState::S0 => Ok(()),
        SleepState::S5 => shutdown(),
        // S1..S4 need _PTS, _WAK and a resume path; none exists.
        _ => Err(AcpiError::PowerStateNotSupported),
    }
}

fn wbinvd() {
    // SAFETY: WBINVD writes back and invalidates the caches; it has no
    // memory operands and is legal at CPL 0, where this runs.
    unsafe { core::arch::asm!("wbinvd", options(nostack, preserves_flags)) };
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
    shutdown_prepared(&prepare_shutdown()?)
}

/// What S5 needs, read ahead: the decoded FADT and the SLP_TYP pair. The
/// panic path reads it before it wipes the heap, so the entry after the wipe
/// takes no lock and reads nothing the wipe may have cleared.
#[derive(Clone, Copy)]
pub struct PreparedOff {
    fadt: FadtInfo,
    types: SleepTypes,
}

/// Read what S5 needs. Takes the ACPI tables' read lock, written only at init.
pub fn prepare_shutdown() -> AcpiResult<PreparedOff> {
    let pair = |p: Option<SleepPackage>| p.map(|p| (p.slp_typ_a, p.slp_typ_b));
    let read = |d: &super::data::AcpiData| (d.fadt, pair(d.s5), pair(d.s5_interp));
    let (fadt, scan, interp) = parser::with_data(read).ok_or(AcpiError::NotInitialized)?;
    let fadt = fadt.ok_or(AcpiError::FadtNotFound)?;
    let ((a, b), _) = choose(scan, interp).ok_or(AcpiError::PowerStateNotSupported)?;
    Ok(PreparedOff { fadt, types: SleepTypes { a, b } })
}

/// Enter S5 from what `prepare_shutdown` read: port writes and a wait,
/// no lock, no allocation and no log.
pub fn shutdown_prepared(off: &PreparedOff) -> AcpiResult<()> {
    // Nothing past this point expects to be interrupted.
    crate::arch::x86_64::interrupt::cli();

    let mut bus = PortBus;
    for _ in 0..2 {
        match enter_sleep(&mut bus, &off.fadt, off.types, wbinvd) {
            Ok(()) => {}
            Err(SleepError::NoControlRegister) => return Err(AcpiError::PowerStateNotSupported),
            Err(SleepError::WriteFailed) => return Err(AcpiError::HardwareAccessFailed),
        }
        // Power should be gone within milliseconds; give slow boards 3 s.
        busy_wait_us(3_000_000);
    }
    Err(AcpiError::HardwareAccessFailed)
}
