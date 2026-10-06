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
mod init;
mod io;
mod mark;
mod protocol;
mod queue;
mod regs;
mod server;
mod setup;
mod transport;
mod vectors;
use nonos_libc::{bring_up, heap_init, mk_debug, mk_exit, EXIT_ABSENT, EXIT_GAVE_UP};
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    // The broker lists every PCI function before the first capsule starts:
    // with no usable virtio-blk there is nothing to wait for.
    if discover::find_virtio_blk().is_none() {
        mk_exit(EXIT_ABSENT);
    }
    // The old loop retried forever, yielding through each backoff rather
    // than sleeping it. The shared bring-up sleeps between a bounded number
    // of attempts, each of which releases what it claimed, and gives up by
    // name. A silent retry reads as a store that never comes up, and
    // everything above it, vfs and the desktop listing, as its own timeout,
    // so each failed attempt still says what failed.
    let attempt = || setup::run().inspect_err(|step| say_stuck(step));
    let Ok(mut driver) = bring_up(b"driver.virtio_blk0", attempt) else {
        mk_exit(EXIT_GAVE_UP);
    };
    server::run(&mut driver);
}

fn say_stuck(step: &str) {
    let mut line = [0u8; 96];
    let tag = b"[BLK] setup stuck: ";
    let n = tag.len();
    line[..n].copy_from_slice(tag);
    let m = step.len().min(line.len() - n);
    line[n..n + m].copy_from_slice(&step.as_bytes()[..m]);
    let _ = mk_debug(line.as_ptr(), n + m);
}
