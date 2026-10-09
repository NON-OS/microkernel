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

use crate::sys::serial;

pub(super) fn init_acpi_tables() {
    if let Some(handoff) = crate::boot::handoff::get_handoff() {
        if let Some(rsdp) = handoff.acpi_rsdp() {
            crate::arch::x86_64::acpi::set_rsdp_address(rsdp);
        }
    }
    match crate::arch::x86_64::acpi::init() {
        Ok(()) => {
            serial::println(b"[NONOS] ACPI tables parsed");
            crate::arch::x86_64::acpi::power_button::init();
        }
        Err(_) => serial::println(b"[NONOS] ACPI init failed; legacy fallbacks engaged"),
    }
    calibrate_against_pm_timer();
}

/*
 * The TSC rate is settled before the ACPI tables are read: CPUID on Intel,
 * the PIT otherwise. AMD parts enumerate no rate, and a laptop with its PIT
 * gated leaves the PIT measurement empty, so the clock would run on a 2.5 GHz
 * guess. The ACPI PM timer exists on every non-reduced platform; once the
 * FADT is known, measure against it and replace the guess.
 */
fn calibrate_against_pm_timer() {
    if crate::time::counter_hz_known() {
        return;
    }
    match crate::arch::x86_64::time::tsc::calibration::pmtimer::calibrate_with_pm_timer() {
        Ok(hz) => {
            crate::time::set_counter_hz_if_unknown(hz);
            crate::sys::timer::tsc::TSC_FREQ_HZ.store(hz, core::sync::atomic::Ordering::SeqCst);
            serial::print(b"[TIMER] TSC measured against the ACPI PM timer: ");
            serial::print_dec(hz / 1_000_000);
            serial::println(b" MHz");
        }
        Err(_) => {
            serial::println(b"[TIMER] no TSC reference (CPUID, PIT, PM timer); rate is a guess")
        }
    }
}
