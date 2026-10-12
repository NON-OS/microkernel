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
//! What the hub class needs of a USB host controller driver and of the
//! driver.usb_hub0: the one walker of every USB hub. It finds hubs on the
//! root ports of every USB host and below them, addresses each other device
//! it finds below a hub, and parks it with the host, which lists it beside
//! its root ports as a virtual port for the class drivers to claim. It
//! holds IPC and Memory only: every request reaches the hardware through a
//! host's wire.

#![no_std]
#![no_main]

extern crate alloc;

mod control;
mod host;
#[path = "../../nonos_usbnet/src/hosts.rs"]
mod hosts;
mod link;
mod ports;
mod run;
mod serve;
#[path = "../../../src/services/registry/usb_hosts.rs"]
mod usb_hosts;
mod wire;

use nonos_libc::{heap_init, mk_exit};

/// # Safety
/// The kernel enters here once, on the capsule's own fresh stack.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    run::run()
}
