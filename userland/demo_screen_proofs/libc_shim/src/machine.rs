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

/// What the kernel's boot census says on this machine, a q35 guest that
/// declares no battery, adapter, lid, thermal zone or touchpad: the eleven
/// rows and the summary, in the kernel's own lines and order
/// (src/hardware/boot_census/run.rs, `text`).
pub const CENSUS: &[u8] = b"[CENSUS] battery absent expected=0 up=0 why=firmware declares none\n\
[CENSUS] ac absent expected=0 up=0 why=firmware declares none\n\
[CENSUS] lid absent expected=0 up=0 why=firmware declares none\n\
[CENSUS] thermal absent expected=0 up=0 why=firmware declares none\n\
[CENSUS] keyboard up expected=1 up=1\n\
[CENSUS] pointer absent expected=0 up=0 why=no touchpad declared\n\
[CENSUS] display up expected=1 up=1\n\
[CENSUS] storage up expected=1 up=1\n\
[CENSUS] network up expected=1 up=1\n\
[CENSUS] usb up expected=1 up=1\n\
[CENSUS] audio up expected=1 up=1\n\
[CENSUS] summary up=7 absent=4 held=0 missing=0 verdict=green\n";

/// MkBootCensus as the kernel answers it (src/syscall/microkernel/boot_census.rs):
/// EINVAL for an empty buffer, else as many whole lines as fit.
pub fn mk_boot_census(out: &mut [u8]) -> i64 {
    if out.is_empty() {
        return -22;
    }
    let fits = if CENSUS.len() <= out.len() {
        CENSUS.len()
    } else {
        CENSUS[..out.len()].iter().rposition(|b| *b == b'\n').map_or(0, |at| at + 1)
    };
    out[..fits].copy_from_slice(&CENSUS[..fits]);
    fits as i64
}
