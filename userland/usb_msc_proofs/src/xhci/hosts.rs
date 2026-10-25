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

//! The USB hosts as the driver finds them (usb_hosts.rs, fastest first):
//! one controller, driver.xhci0, unless the test says none has registered;
//! the others never register here.

#[path = "../../../nonos_usbnet/src/hosts.rs"]
mod hosts;
#[path = "../../../nonos_usbnet/src/hosts_first.rs"]
mod hosts_first;
#[path = "../../../../src/services/registry/usb_hosts.rs"]
mod usb_hosts;

pub use self::hosts::Hosts;
use super::device::setup;
use usb_hosts::USB_HOSTS;

pub fn hosts<T>(keep: impl FnMut() -> T) -> Hosts<T> {
    Hosts::new(USB_HOSTS, keep)
}

pub fn host(name: &str) -> Option<u32> {
    if name != USB_HOSTS[0] {
        return None;
    }
    setup(|d| {
        d.lookups += 1;
        (!d.unregistered).then_some(1)
    })
}
