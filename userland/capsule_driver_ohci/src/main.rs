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

//! driver.ohci0: an OHCI host controller served on the host wire that
//! driver.xhci0 serves, so the USB class drivers reach full- and low-speed
//! devices on OHCI hosts: VirtualBox's, QEMU's, and AMD and Apple chipsets'.

#![no_std]
#![no_main]

mod devices;
mod env;
mod facts;
mod hc;
mod platform;
mod run;
mod say;
mod sched;
mod server;
mod setup;
mod wire;
mod xfer;

use nonos_libc::{mk_exit, start_driver};

const DRIVER: &[u8] = b"driver.ohci";

/// Without an OHCI controller the driver says so and leaves before claiming
/// anything; one that is there is started and then served.
///
/// # Safety
/// The kernel enters here once, on the capsule's own fresh stack.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    match start_driver(DRIVER, setup::find(), |dev| setup::run(*dev)) {
        Ok(driver) => {
            say::started(&driver);
            run::run(driver)
        }
        Err(code) => mk_exit(code),
    }
}
