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

//! The configuration Linux would run a listed device in, and in it the
//! interface products[] matches: class, subclass and protocol ff/ff/00
//! (USB_DEVICE_AND_INTERFACE_INFO) with a bulk IN and a bulk OUT pipe.

use nonos_usbnet::desc::{config_header, interfaces, CLASS_VENDOR};
use nonos_usbnet::{Found, Pipes};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AxFunction {
    pub config: u8,
    pub interface: u8,
    pub alt: u8,
    pub pipes: Pipes,
}

pub fn find_function(found: &Found) -> Option<AxFunction> {
    let raw = chosen(found)?;
    let (_, config) = config_header(raw)?;
    let ours = |i: &&nonos_usbnet::desc::Interface| {
        (i.class, i.subclass, i.protocol) == (CLASS_VENDOR, 0xff, 0) && i.has_bulk_pair()
    };
    let i = *interfaces(raw).iter().find(ours)?;
    Some(AxFunction { config, interface: i.number, alt: i.alt, pipes: i.pipes })
}

/// usb_choose_configuration (Linux drivers/usb/core/generic.c): a device
/// not of the vendor class runs in its first configuration whose first
/// interface is not vendor specific, so a class driver takes it; failing
/// that, in its first configuration.
fn chosen(found: &Found) -> Option<&alloc::vec::Vec<u8>> {
    let class_first = |raw: &&alloc::vec::Vec<u8>| {
        interfaces(raw).first().is_some_and(|i| i.class != CLASS_VENDOR)
    };
    let class = (found.info.class != CLASS_VENDOR)
        .then(|| found.configs.iter().find(class_first))
        .flatten();
    class.or(found.configs.first())
}
