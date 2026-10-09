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

use super::state::HPET_BASE;

/// The mapped HPET base, from the ACPI HPET table only (see `time::hpet`).
pub(crate) fn detect_hpet() -> Option<u64> {
    super::super::hpet::detect_hpet()
}

/// Whether `base` is the HPET block `detect_hpet` mapped. A physical
/// address is never read here: it is only compared.
pub fn is_valid_hpet_base(base: u64) -> bool {
    base != 0 && detect_hpet() == Some(base)
}

pub(crate) fn configure_hpet_for_timing(hpet_base: u64) {
    unsafe {
        let capabilities = core::ptr::read_volatile(hpet_base as *const u64);
        let period_fs = (capabilities >> 32) as u32;
        let config_reg = (hpet_base + 0x10) as *mut u64;
        core::ptr::write_volatile(config_reg, 0);
        let counter_reg = (hpet_base + 0xF0) as *mut u64;
        core::ptr::write_volatile(counter_reg, 0);
        core::ptr::write_volatile(config_reg, 1);
        if let Some(logger) = crate::log::logger::try_get_logger() {
            if let Some(log_mgr) = logger.lock().as_mut() {
                log_mgr.log(
                    crate::log::Severity::Info,
                    &alloc::format!("[TIMER] HPET configured, period: {} fs", period_fs),
                );
            }
        }
    }
}

pub(crate) fn configure_hpet(hpet_base: u64, freq_hz: u32) {
    unsafe {
        let capabilities = core::ptr::read_volatile(hpet_base as *const u64);
        let counter_period = (capabilities >> 32) as u32;
        let ticks_per_interrupt =
            (1_000_000_000_000_000u64 / counter_period as u64) / freq_hz as u64;
        let timer0_config_addr = (hpet_base + 0x100) as *mut u64;
        let timer0_comparator_addr = (hpet_base + 0x108) as *mut u64;
        core::ptr::write_volatile(timer0_config_addr, 0x004C);
        core::ptr::write_volatile(timer0_comparator_addr, ticks_per_interrupt);
        let general_config_addr = (hpet_base + 0x010) as *mut u64;
        core::ptr::write_volatile(general_config_addr, 1);
    }
}

pub fn get_hpet_counter() -> Option<u64> {
    let hpet_base = HPET_BASE.load(Ordering::Relaxed);
    if hpet_base == 0 {
        return None;
    }
    unsafe {
        let counter_reg = (hpet_base + 0xF0) as *const u64;
        Some(core::ptr::read_volatile(counter_reg))
    }
}

pub fn hpet_to_ns(hpet_ticks: u64) -> Option<u64> {
    let hpet_base = HPET_BASE.load(Ordering::Relaxed);
    if hpet_base == 0 {
        return None;
    }
    unsafe {
        let capabilities = core::ptr::read_volatile(hpet_base as *const u64);
        let period_fs = (capabilities >> 32) as u32;
        Some((hpet_ticks * period_fs as u64) / 1_000_000)
    }
}
