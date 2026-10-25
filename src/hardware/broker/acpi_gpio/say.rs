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

use crate::arch::firmware_table::devices::GpioController;
use crate::sys::serial;

use super::p2sb::sbreg_bar;

/// One `[GPIO]` line naming the controller, so `log GPIO` shows on one
/// photo which layout the touchpad's interrupt line will be read through.
pub(super) fn say(ctl: &GpioController, what: &str) {
    let hid: alloc::string::String =
        ctl.hid.iter().take_while(|&&b| b != 0).map(|&b| b as char).collect();
    let name: alloc::string::String = ctl.name.iter().map(|&b| b as char).collect();
    let line = format!("[GPIO] {} {} uid {}: {}", hid, name, ctl.uid, what);
    serial::println(line.as_bytes());
}

/// SBREG_BAR read once, with its value or its absence on the log.
pub(super) fn read_sbreg() -> Option<u64> {
    let bar = sbreg_bar();
    match bar {
        Some(b) if census() => serial::println(format!("[GPIO] SBREG_BAR {:#x}", b).as_bytes()),
        Some(_) => serial::println(b"[GPIO] SBREG_BAR read"),
        None => serial::println(b"[GPIO] SBREG_BAR not readable from 00:1f.1"),
    }
    bar
}

/// How many community windows, and, in a census build, where the first is.
pub(super) fn windows_line(ctl: &GpioController) -> alloc::string::String {
    match census() {
        true => format!("{} community windows, first at {:#x}", ctl.window_count, ctl.windows[0].0),
        false => format!("{} community windows", ctl.window_count),
    }
}

/// A physical address reaches the log only in a NONOS_DEVICE_CENSUS=1 build,
/// the touchpad bring-up build census/render.rs keys on.
fn census() -> bool {
    matches!(option_env!("NONOS_DEVICE_CENSUS"), Some("1"))
}
