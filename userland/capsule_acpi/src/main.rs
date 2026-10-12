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

#![no_std]
#![no_main]

//! capsule_acpi: the platform's firmware, run in NONOS's own interpreter
//! where a broken or hostile table can stop this capsule and nothing else.

extern crate alloc;

mod aei;
mod apei;
mod backlight;
mod boot;
mod buttons;
mod cooling;
mod cpu;
mod dpower;
mod ec;
mod events_io;
mod host;
mod hotkeys;
mod legacy;
mod numa;
mod platform;
mod ports;
mod props;
mod reserved;
mod power;
mod serial;
mod server;
mod sleep;
mod smbios;
mod suspend;
mod sys;
mod typec;

use nonos_libc::{heap_init, mk_exit};

/// Exit code when the kernel hands over no FADT or DSDT: nothing to run.
const EXIT_NO_TABLES: i32 = 2;

/// # Safety
/// The capsule entry point. The kernel loader calls this once on a fresh
/// stack with the heap region reserved; it must never be called from Rust.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    match boot::boot() {
        Some(acpi) => server::run(acpi),
        None => mk_exit(EXIT_NO_TABLES),
    }
}
