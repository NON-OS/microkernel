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

extern crate alloc;

mod chip;
mod constants;
mod discover;
mod hw;
mod init;
mod link;
mod log;
mod protocol;
mod queue;
mod regmap;
mod regs;
mod rx;
mod server;
mod setup;
mod tx;

use nonos_libc::{heap_init, mk_exit, start_driver};

const DRIVER: &[u8] = b"driver.rtl8169";
const EXIT_HEAP_INIT: i32 = 1;

/// Without a card the driver says so and leaves (`EXIT_ABSENT`) before
/// claiming anything. With one, each attempt takes the grants and programs
/// the part, giving every grant back when either half fails; the attempts
/// are bounded and slept between, and running out is `EXIT_GAVE_UP`.
///
/// # Safety
///
/// Only the kernel's capsule loader enters here, once, as the first code the
/// process runs, on its first thread: with the image's segments mapped, its
/// relative relocations and RELRO applied (`load_elf_entry_into`), and on a
/// fresh user stack with rsp ≡ 8 (mod 16) before the prologue, as a call
/// leaves it and as an `extern "C"` fn expects (`allocate_user_stack` hands
/// back its top less 8). It must never be called from Rust.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(EXIT_HEAP_INIT);
    }
    let started = start_driver(DRIVER, discover::find_rtl8169(), |dev| {
        setup::run(*dev).and_then(init::finish)
    });
    let mut driver = match started {
        Ok(d) => d,
        Err(code) => mk_exit(code),
    };
    server::run(&mut driver);
}
