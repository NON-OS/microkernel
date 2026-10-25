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

//! The machine the demo is recorded on: its processes, the display mode
//! shown and the driver showing it.

use std::cell::Cell;

/// About's own pid.
pub const OWN_PID: u32 = 41;

/// (pid, name, capability mask) of every process, in pid order.
pub const PROCS: [(u32, &str, u64); 9] = [
    (1, "init", 0x7FFF),
    (7, "compositor", 0x39018),
    (9, "wm", 0x18),
    (12, "input_router", 0x1018),
    (14, "driver.virtio_gpu0", 0x3B018),
    (21, "desktop_shell", 0x19018),
    (33, "nonos-terminal", 0x18),
    (OWN_PID, "nonos-about", 0x80019),
    (52, "nonos-settings", 0x19018),
];

thread_local! {
    pub static MODE: Cell<(u32, u32)> = const { Cell::new((1920, 1080)) };
    pub static DRIVER: Cell<&'static str> = const { Cell::new("driver.virtio_gpu0") };
}

/// Show `w` x `h` through the driver announced as `driver`.
pub fn show(w: u32, h: u32, driver: &'static str) {
    MODE.with(|m| m.set((w, h)));
    DRIVER.with(|d| d.set(driver));
}

/// A process's name, the shown driver standing in for whichever is listed.
pub fn name_of(name: &'static str) -> &'static str {
    if name.starts_with("driver.") {
        DRIVER.with(Cell::get)
    } else {
        name
    }
}
