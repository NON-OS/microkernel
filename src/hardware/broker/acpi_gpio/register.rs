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

use alloc::format;

use crate::arch::x86_64::acpi::aml::{
    community_window, enumerate_gpio_controllers, GpioController,
};
use crate::sys::serial;

use super::super::table::register_platform_device;
use super::p2sb::sbreg_bar;
use super::record::device_record;

/// Register the ACPI-enumerated GPIO controllers with the broker table, one
/// record per controller with a bar per community. A window need not be
/// page aligned (the AMD bank sits at 0xFED81500): the MMIO grant maps the
/// pages around it as it does for a 2 KiB AHCI ABAR (broker/mmio/window.rs).
pub fn register_acpi_gpio() {
    let mut sbreg: Option<Option<u64>> = None;
    for mut ctl in enumerate_gpio_controllers() {
        if ctl.window_count == 0 {
            let Some(base) = *sbreg.get_or_insert_with(read_sbreg) else {
                say(&ctl, "SBREG_BAR unreadable at 00:1f.1, no community windows");
                continue;
            };
            let (pids, n) = (ctl.pids, ctl.pid_count);
            for &pid in &pids[..n] {
                let (at, len) = community_window(base, pid);
                ctl.push_window(at, len);
            }
        }
        let first = ctl.windows[0].0;
        say(&ctl, &format!("{} community windows, first at {:#x}", ctl.window_count, first));
        register_platform_device(device_record(&ctl));
    }
}

/// One `[GPIO]` line naming the controller, so `log GPIO` shows on one
/// photo which layout the touchpad's interrupt line will be read through.
fn say(ctl: &GpioController, what: &str) {
    let hid: alloc::string::String =
        ctl.hid.iter().take_while(|&&b| b != 0).map(|&b| b as char).collect();
    let name: alloc::string::String = ctl.name.iter().map(|&b| b as char).collect();
    let line = format!("[GPIO] {} {} uid {}: {}", hid, name, ctl.uid, what);
    serial::println(line.as_bytes());
}

/// SBREG_BAR read once, with its value or its absence on the log.
fn read_sbreg() -> Option<u64> {
    let bar = sbreg_bar();
    match bar {
        Some(b) => serial::println(format!("[GPIO] SBREG_BAR {:#x}", b).as_bytes()),
        None => serial::println(b"[GPIO] SBREG_BAR not readable from 00:1f.1"),
    }
    bar
}
