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

//! The SD/eMMC hosts the broker found through ACPI, which MkDmaMap grants
//! frames (`acpi_stop`) and which have no Bus Master to clear (F-128). A
//! claim's stop resets one (`sdhci_stop::reset_all`) before any of its
//! grants is torn down; one whose reset did not read back done keeps its
//! released frames for good (teardown.rs), never freed under its DMA.

extern crate alloc;

use alloc::vec::Vec;
use spin::Mutex;

use super::acpi_stop::{acpi_stop, Stop, BUS_ACPI};
use super::sd_window::Regs;
use super::sdhci_stop::{frees, reset_all, Outcome, SOFTWARE_RESET};
use crate::memory::addr::PhysAddr;

const SPAN: u64 = 0x1000; // the first page, where every register the stop reads lies

/// Each host added and its registers; None when fenced or not mapped.
static HOSTS: Mutex<Vec<(u64, Option<Regs>)>> = Mutex::new(Vec::new());
/// The hosts whose last reset did not read back done.
static RUNNING: Mutex<Vec<u64>> = Mutex::new(Vec::new());

/// An SD host the ACPI broker added, with its first register window unless
/// a departed device's fence lies over it.
pub fn note_sd_host(device_id: u64, window: Option<(u64, u64)>) {
    let span = |size: u64| size.min(SPAN);
    let regs = window.filter(|&(_, size)| size > SOFTWARE_RESET).and_then(|(base, size)| {
        let va = crate::memory::mmio::map_device_memory(PhysAddr::new(base), span(size) as usize);
        va.ok().map(|va| Regs::new(va.as_u64(), span(size)))
    });
    HOSTS.lock().push((device_id, regs));
}

/// Stop `device_id` when it is an ACPI kind with a stop, and say so; false
/// for any other device, which the caller stops its own way. A host it
/// cannot reach is taken as still running.
pub(in crate::hardware::broker) fn stop_sd_host(device_id: u64) -> bool {
    let Some((bus, class)) = super::super::table::kind_of(device_id) else { return false };
    if bus != BUS_ACPI || acpi_stop(class) != Stop::SdhciResetAll {
        return false;
    }
    let regs = HOSTS.lock().iter().find(|h| h.0 == device_id).and_then(|h| h.1);
    let outcome = regs.map_or(Outcome::Unreachable, |mut r| reset_all(&mut r));
    let mut running = RUNNING.lock();
    running.retain(|&d| d != device_id);
    if !frees(outcome) {
        running.push(device_id);
        crate::sys::serial::print(b"[DMA] SD host ");
        crate::sys::serial::print_hex(device_id);
        crate::sys::serial::println(b" did not stop; its released frames are kept");
    }
    true
}

/// Whether `device_id`'s last reset did not read back done.
pub(super) fn running(device_id: u64) -> bool {
    RUNNING.lock().contains(&device_id)
}
