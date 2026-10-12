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

//! Finding the USB hosts: the names in usb_hosts.rs, fastest first, kept
//! in a `Hosts` (nonos_usbnet's hosts.rs) with the devices each serves.

use nonos_libc::mk_service_lookup;

use super::hosts::Hosts;
use super::usb_hosts::USB_HOSTS;

/// Every host, none looked up yet, each with a fresh `keep()`.
pub fn hosts<T>(keep: impl FnMut() -> T) -> Hosts<T> {
    Hosts::new(USB_HOSTS, keep)
}

/// The port of the host registered as `name`, once it has registered.
pub fn host(name: &str) -> Option<u32> {
    let (mut port, mut pid) = (0u32, 0u32);
    let rc = mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid);
    (rc == 0).then_some(port)
}
