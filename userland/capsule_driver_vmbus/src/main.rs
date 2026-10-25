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

//! NONOS userland: the VMBus core, Hyper-V's device bus. It contacts the
//! host through the kernel's SynIC calls, takes the host's channel offers,
//! and keeps the bus: offers added later are kept, withdrawn ones released.
//! A storage or network channel is given a record of its own for its driver
//! (`give`); the core opens no channel.

#![no_std]
#![no_main]

mod connect;
mod constants;
mod discover;
mod give;
mod guid;
mod hv;
mod msg;
mod offers;
mod say;
mod serve;
mod setup;
mod wait;

use nonos_libc::{mk_exit, start_driver};

const DRIVER: &[u8] = b"driver.vmbus";

/// Without the VMBus root (no Hyper-V SynIC) the core says so and leaves.
/// With it, each attempt claims the root, contacts the host and gathers
/// the offers, giving everything back when a step fails; the attempts are
/// bounded and slept between.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    let mut driver = match start_driver(DRIVER, discover::find(), |id| setup::run(*id)) {
        Ok(d) => d,
        Err(code) => mk_exit(code),
    };
    say::up(driver.bus, &driver.offers);
    serve::run(&mut driver);
}
