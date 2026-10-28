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

//! The vendor configuration. These adapters offer it and, most of them,
//! a CDC-ECM one beside it, in either order; Linux v6.12 takes the first
//! configuration whose first interface is of the vendor class (FFh), and
//! so does this driver, when that interface at alternate 0 carries a bulk
//! IN and a bulk OUT. One without it is left to cdc_ecm.

use alloc::vec::Vec;

use nonos_usbnet::desc::{config_header, interfaces, CLASS_VENDOR};
use nonos_usbnet::{Found, Pipes};

pub struct Vendor<'a> {
    /// bConfigurationValue.
    pub value: u8,
    pub pipes: Pipes,
    /// The configuration's descriptors, for the interfaces to hold.
    pub raw: &'a Vec<u8>,
}

pub fn vendor_config(found: &Found) -> Option<Vendor<'_>> {
    found.configs.iter().find_map(|raw| {
        let (_, value) = config_header(raw)?;
        let first = *interfaces(raw).first()?;
        let ours = first.class == CLASS_VENDOR && first.alt == 0 && first.has_bulk_pair();
        ours.then_some(Vendor { value, pipes: first.pipes, raw })
    })
}
