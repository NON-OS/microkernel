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

//! TSC calibration against the ACPI PM timer, the reference Linux falls back
//! to (`pit_hpet_ptimer_calibrate_cpu`) when the PIT does not answer. AMD
//! parts enumerate no TSC frequency, some boards have no HPET, and some
//! recent laptops gate the PIT, but every non-reduced ACPI platform has the
//! PM timer: a 3.579545 MHz counter in the FADT's PM_TMR_BLK.

use super::super::asm::rdtsc_unserialized;
use super::super::constants::{DEFAULT_CALIBRATION_MS, MAX_FREQUENCY, MIN_FREQUENCY};
use super::super::error::{TscError, TscResult};
use super::math::{hz_from_reference, pm_timer_delta, reference_timeout_ticks, PM_TIMER_HZ};
use crate::arch::x86_64::acpi::hw::gas::{gas_read, Gas};
use crate::arch::x86_64::acpi::hw::port_bus::PortBus;

const FLAG_TMR_VAL_EXT: u32 = 1 << 8;
const SAMPLES: usize = 3;

fn pm_timer() -> Option<(Gas, bool)> {
    crate::arch::x86_64::acpi::parser::with_data(|d| {
        let f = d.fadt?;
        if f.is_hw_reduced() || !f.pm_tmr.is_accessible() {
            return None;
        }
        Some((f.pm_tmr, f.flags & FLAG_TMR_VAL_EXT != 0))
    })
    .flatten()
}

/// Measure the TSC against the PM timer. Needs the ACPI tables parsed.
pub fn calibrate_with_pm_timer() -> TscResult<u64> {
    let (gas, ext) = pm_timer().ok_or(TscError::NoReferenceTimer)?;
    let mut bus = PortBus;
    let mut read = || gas_read(&mut bus, &gas).map(|v| v as u32);
    let window = PM_TIMER_HZ * DEFAULT_CALIBRATION_MS as u64 / 1000;
    let timeout = reference_timeout_ticks(DEFAULT_CALIBRATION_MS as u64, MAX_FREQUENCY);

    let mut samples = [0u64; SAMPLES];
    let mut n = 0;
    for _ in 0..SAMPLES {
        // Start on a tick edge so the partial first tick does not count.
        let first = read().ok_or(TscError::NoReferenceTimer)?;
        let edge_start = rdtsc_unserialized();
        let mut p0 = first;
        while p0 == first {
            if rdtsc_unserialized().wrapping_sub(edge_start) > timeout {
                return Err(TscError::NoReferenceTimer);
            }
            p0 = read().ok_or(TscError::NoReferenceTimer)?;
        }
        let t0 = rdtsc_unserialized();
        let (mut p1, mut t1);
        loop {
            p1 = read().ok_or(TscError::NoReferenceTimer)?;
            t1 = rdtsc_unserialized();
            if pm_timer_delta(p0, p1, ext) as u64 >= window {
                break;
            }
            if t1.wrapping_sub(t0) > timeout {
                return Err(TscError::NoReferenceTimer);
            }
        }
        let ticks = pm_timer_delta(p0, p1, ext) as u64;
        if let Some(hz) = hz_from_reference(t1.wrapping_sub(t0), ticks, PM_TIMER_HZ) {
            if (MIN_FREQUENCY..=MAX_FREQUENCY).contains(&hz) {
                samples[n] = hz;
                n += 1;
            }
        }
    }
    if n == 0 {
        return Err(TscError::CalibrationFailed);
    }
    samples[..n].sort_unstable();
    Ok(samples[n / 2])
}
