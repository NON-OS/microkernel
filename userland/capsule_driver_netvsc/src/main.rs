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
mod host;
mod link;
mod nic;
mod nvsp;
mod probe;
mod rndis;
mod serve;
mod setup;
mod start;
mod station;

use nonos_libc::{heap_init, mk_exit, start_driver};

const DRIVER: &[u8] = b"driver.netvsc";
/// The service name the capsule is spawned under.
pub const NAME: &[u8] = b"driver.netvsc0";
const EXIT_HEAP_INIT: i32 = 1;
/// The host broke the channel: everything is given back.
const EXIT_CHANNEL_BROKEN: i32 = 2;

/// Without a network channel given by the VMBus core in time, the driver
/// says so and leaves before claiming anything. With one, each attempt opens
/// the channel, starts the protocol and settles the station address, giving
/// everything back when a step fails; the attempts are bounded and slept
/// between. Then it serves frames until the host breaks the channel, and
/// gives everything back.
///
/// # Safety
/// The kernel enters here once, on the capsule's own fresh stack.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(EXIT_HEAP_INIT);
    }
    let mut driver = match start_driver(DRIVER, discover::find(), |dev| setup::run(*dev)) {
        Ok(d) => d,
        Err(code) => mk_exit(code),
    };
    serve::run(&mut driver.net);
    driver.release();
    mk_exit(EXIT_CHANNEL_BROKEN);
}
