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

//! Which source gave the APs their TSC rate, said once on the log by the
//! boot CPU after the APs are started. An AP picks the rate with
//! interrupts masked, where taking the serial lock could spin against the
//! boot CPU's printing (see smp/ap/go_live.rs), so it only records here.

use super::frequency_api::get_tsc_frequency;
use super::frequency_pit::FALLBACK_HZ;
use core::sync::atomic::{AtomicU8, Ordering};

pub(super) const FROM_BOOT_CPU: u8 = 1;
pub(super) const FROM_CPUID: u8 = 2;
pub(super) const FROM_PIT: u8 = 3;

pub(super) static SOURCE: AtomicU8 = AtomicU8::new(0);

/// `[SMP] AP TSC rate 2994 MHz, the boot CPU's`, or where else it came from.
pub fn report_ap_tsc_rate() {
    let mhz = get_tsc_frequency() / 1_000_000;
    let mut l = crate::sys::serial::Line::new();
    l.str(b"[SMP] AP TSC rate ").dec(mhz).str(b" MHz, ");
    match SOURCE.load(Ordering::Acquire) {
        FROM_BOOT_CPU => l.str(b"the boot CPU's"),
        FROM_CPUID => l.str(b"from CPUID on the first AP"),
        FROM_PIT if get_tsc_frequency() == FALLBACK_HZ => {
            l.str(b"assumed: the PIT gave the first AP no usable count")
        }
        FROM_PIT => l.str(b"measured against the PIT on the first AP"),
        _ => l.str(b"none recorded: no AP reached cpu::init_ap"),
    };
    l.end();
}
