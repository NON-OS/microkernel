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

mod clock;
mod constants;
mod discover;
mod fill;
mod init;
mod protocol;
mod queue;
mod regs;
mod scrub;
mod server;
mod setup;
mod transport;

use nonos_libc::{bring_up, heap_init, mk_exit, EXIT_ABSENT, EXIT_GAVE_UP};

/// # Safety
///
/// The capsule's entry point: the kernel calls it once, on a fresh stack,
/// and nothing else may.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }

    // The broker lists every PCI function before the first capsule starts:
    // with no virtio-rng there is nothing to wait for.
    if discover::find_virtio_rng().is_none() {
        mk_exit(EXIT_ABSENT);
    }

    // A present device that will not come up is tried a bounded number of
    // times, each failed try releasing what it claimed, then given up on.
    let Ok(mut driver) = bring_up(b"driver.virtio_rng", setup::run) else {
        mk_exit(EXIT_GAVE_UP);
    };

    // One fill before serving: a device answering only zero bytes is not a
    // source. A device that never answers is reset before it is given back.
    let mut probe = [0u8; 32];
    let expired = clock::expired_after(constants::FILL_MS);
    let filled = fill::fill(driver.transport, &mut driver.queue, &mut probe, expired);
    let live = matches!(filled, Ok(n) if probe.iter().take(n).any(|&b| b != 0));
    scrub::scrub(&mut probe);
    if !live {
        let _ = driver.quiesce();
        mk_exit(if filled.is_ok() { 4 } else { 3 });
    }
    server::run(&mut driver);
}
