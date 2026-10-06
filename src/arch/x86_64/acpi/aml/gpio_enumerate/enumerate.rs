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

use alloc::vec::Vec;

use super::super::scan::find_devices;
use super::super::tables;
use super::super::types::GpioController;
use super::crs::parse_gpio_crs;
use super::hid_match::hid_is_gpio_controller;
use super::sideband::community_pids;
use super::uid::parse_uid;

/// Enumerate GPIO controllers from the ACPI DSDT/SSDT, returning those with
/// a static window in `_CRS` or a sideband entry to compute theirs from.
/// Empty on any failure; never panics.
pub fn enumerate_gpio_controllers() -> Vec<GpioController> {
    let mut out = Vec::new();
    for aml in tables::aml_blocks() {
        for scope in find_devices(&aml, hid_is_gpio_controller) {
            let mut ctl = GpioController::new(scope.hid);
            ctl.name = scope.name;
            ctl.uid = parse_uid(scope.body).unwrap_or(0);
            let pids = community_pids(&scope.hid);
            ctl.pid_count = pids.len().min(ctl.pids.len());
            ctl.pids[..ctl.pid_count].copy_from_slice(&pids[..ctl.pid_count]);
            if let Some(crs) = scope.crs {
                parse_gpio_crs(crs, &mut ctl);
            }
            // A `_CRS` method built from named templates keeps its descriptors
            // in the device body; scan there when the scoped parse found no
            // window, mirroring the I2C-HID enumerator.
            if ctl.window_count == 0 {
                parse_gpio_crs(scope.body, &mut ctl);
            }
            if ctl.is_valid() {
                out.push(ctl);
            }
        }
    }
    out
}
