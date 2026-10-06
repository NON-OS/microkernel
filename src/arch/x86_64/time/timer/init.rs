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

use core::sync::atomic::Ordering;

use super::hpet::{configure_hpet, configure_hpet_for_timing, detect_hpet};
use super::state::{ACTIVE_TIMERS, BOOT_TIME, HPET_BASE, TIMER_INITIALIZED, TSC_FREQUENCY};
use super::tsc::rdtsc;

pub fn init_boot_time() {
    if BOOT_TIME.load(Ordering::Relaxed) == 0 {
        BOOT_TIME.store(rdtsc(), Ordering::SeqCst);
    }
}

pub fn init() {
    init_boot_time();
    let tsc_freq = calibrate_tsc_frequency();
    TSC_FREQUENCY.store(tsc_freq, Ordering::SeqCst);
    if let Some(hpet_base) = detect_hpet() {
        HPET_BASE.store(hpet_base, Ordering::SeqCst);
        configure_hpet_for_timing(hpet_base);
    }
    ACTIVE_TIMERS.lock().clear();
    TIMER_INITIALIZED.store(true, Ordering::SeqCst);
    if let Some(logger) = crate::log::logger::try_get_logger() {
        if let Some(log_mgr) = logger.lock().as_mut() {
            log_mgr.log(
                crate::log::nonos_logger::Severity::Info,
                &alloc::format!("[TIMER] Initialized with TSC frequency: {} Hz", tsc_freq),
            );
        }
    }
}

/// CPUID's enumerated rate, else a bounded PIT measurement, else 0. The PIT
/// loop used to spin on OUT2 with no timeout, which hangs for ever on a
/// laptop whose 8254 is gated off.
fn calibrate_tsc_frequency() -> u64 {
    if let Some(hz) = crate::arch::x86_64::time::tsc::get_cpuid_frequency() {
        return hz;
    }
    crate::arch::x86_64::time::tsc::calibrate_with_pit().map(|(hz, _)| hz).unwrap_or(0)
}

pub fn init_with_freq(freq_hz: u32) {
    BOOT_TIME.store(rdtsc(), Ordering::SeqCst);
    unsafe {
        let divisor = 1193182 / freq_hz;
        crate::arch::x86_64::port::outb(0x43, 0x36);
        crate::arch::x86_64::port::outb(0x40, (divisor & 0xFF) as u8);
        crate::arch::x86_64::port::outb(0x40, ((divisor >> 8) & 0xFF) as u8);
    }
    if let Some(hpet_base) = detect_hpet() {
        configure_hpet(hpet_base, freq_hz);
        crate::log_info!("HPET configured at frequency {} Hz", freq_hz);
    } else {
        crate::log_info!("PIT configured at frequency {} Hz", freq_hz);
    }
}
