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

use crate::arch::firmware_table::devices::{community_window, enumerate_gpio_controllers};

use super::super::acpi::platform_paths::keep_unpathed;
use super::super::table::register_platform_device;
use super::record::device_record;
use super::say::{read_sbreg, say, windows_line};

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
        say(&ctl, &windows_line(&ctl));
        let id = register_platform_device(device_record(&ctl));
        keep_unpathed(id, ctl.hid, u64::from(ctl.uid));
    }
}
