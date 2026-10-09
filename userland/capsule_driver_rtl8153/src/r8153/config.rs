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

//! The vendor configuration, as Linux rtl8152_cfgselector_probe picks it:
//! the configuration whose first interface is vendor specific (class
//! 0xFF). r8152 then wants its bulk IN on endpoint 1 and its bulk OUT on
//! endpoint 2 (rtl_check_vendor_ok); the interrupt endpoint 3 beside them
//! is not used here, as driver.xhci0 configures only the bulk pair.

use nonos_usbnet::desc::{config_header, interfaces, CLASS_VENDOR};
use nonos_usbnet::Pipes;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct VendorConfig {
    /// bConfigurationValue, for SET_CONFIGURATION.
    pub value: u8,
    pub pipes: Pipes,
}

pub fn vendor_config(raw: &[u8]) -> Option<VendorConfig> {
    let (_, value) = config_header(raw)?;
    let first = *interfaces(raw).first()?;
    let endpoints = first.pipes.bulk_in & 0x0F == 1 && first.pipes.bulk_out & 0x0F == 2;
    (first.class == CLASS_VENDOR && endpoints).then_some(VendorConfig { value, pipes: first.pipes })
}
