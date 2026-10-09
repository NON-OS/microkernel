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

mod constants;
mod discover;
mod fill;
mod init;
mod protocol;
mod queue;
mod regs;
mod server;
mod setup;
mod transport;

use nonos_libc::{bring_up, heap_init, mk_exit, EXIT_ABSENT, EXIT_GAVE_UP};

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }

    /*
     * The broker lists every PCI function before the first capsule starts:
     * with no virtio-rng there is nothing to wait for. Discovery used to be
     * retried with everything else for ten seconds of yields.
     */
    if discover::find_virtio_rng().is_none() {
        mk_exit(EXIT_ABSENT);
    }

    /*
     * A present device that will not come up is tried a bounded number of
     * times with a sleep between tries, each failed try releasing what it
     * claimed, and then given up on once, by name.
     */
    let Ok(mut driver) = bring_up(b"driver.virtio_rng", setup::run) else {
        mk_exit(EXIT_GAVE_UP);
    };

    match crate::fill::fill(driver.transport, &mut driver.queue) {
        Ok(n) => {
            let bytes = driver.queue.buffer(n);
            let mut nz = 0usize;
            for &b in bytes.iter() {
                if b != 0 {
                    nz += 1;
                }
            }
            if nz == 0 {
                driver.release();
                mk_exit(4);
            }
        }
        Err(_) => {
            driver.release();
            mk_exit(3);
        }
    }

    let _ = driver.queue.region_phys();
    let _ = driver.claim_epoch;
    server::run(&mut driver);
}
