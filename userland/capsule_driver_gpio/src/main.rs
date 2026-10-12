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

//! The x86 GPIO controller capsule: every GPIO controller the kernel
//! published is claimed and its windows mapped, the `_AEI` pins and the
//! serial-bus devices' GpioInt pins armed as firmware describes them, and
//! the controller's pending pins taken apart for capsule_acpi and each
//! device's holder (`nonos_gpio` holds the engine).

#![no_std]
#![no_main]

extern crate alloc;

mod acpi;
mod aei;
mod bank;
mod ctl;
mod discover;
mod mmio;
mod pin_requests;
mod pins;
mod queue;
mod requests;
mod say;
mod serve;

use nonos_libc::{heap_init, mk_exit, say_absent, EXIT_ABSENT};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let _ = heap_init();
    let mut banks = discover::banks();
    if banks.is_empty() {
        say_absent(b"driver.gpio0");
        mk_exit(EXIT_ABSENT);
    }
    for b in banks.iter_mut() {
        pins::arm(b);
    }
    serve::run(banks)
}
