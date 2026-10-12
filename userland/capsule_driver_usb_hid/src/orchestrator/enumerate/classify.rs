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

//! Reading an addressed device's configuration and binding its HID
//! interfaces. A hub is driver.usb_hub0's: its slot is given back, and the
//! devices below it come back as virtual ports this driver claims as it
//! claims root ports.

use alloc::vec::Vec;

use crate::descriptors::{configuration_value, hid_bindings, is_hub};
use crate::xhci::{claim_interface, control_transfer, MAX_DESCRIPTOR_LEN};

use super::super::binding::configure_binding;
use super::read_configuration::read_configuration;
use super::reports::sort_by_report;
use super::types::{HidEndpoint, Outcome};

const SET_CONFIGURATION: u8 = 0x09;

/// Bind `slot`'s HID interfaces, on port `port`, onto `eps`.
pub(super) fn classify(xhci_port: u32, slot: u8, port: u8, eps: &mut Vec<HidEndpoint>) -> Outcome {
    let mut desc = [0u8; MAX_DESCRIPTOR_LEN];
    let Some(len) = read_configuration(xhci_port, slot, &mut desc) else {
        return Outcome::Failed;
    };
    let raw = &desc[..len];
    let bindings = match hid_bindings(raw) {
        _ if is_hub(raw) => return Outcome::NotHid,
        Ok(b) if !b.is_empty() => b,
        _ => return Outcome::NotHid,
    };
    let value = configuration_value(raw).unwrap_or(1) as u16;
    let mut dummy = [0u8; 0];
    if control_transfer(xhci_port, slot, 0x00, SET_CONFIGURATION, value, 0, 0, &mut dummy).is_err()
    {
        return Outcome::Failed;
    }
    let bindings = sort_by_report(xhci_port, slot, bindings);
    if bindings.is_empty() {
        return Outcome::NotHid;
    }
    for (binding, pad) in bindings {
        let before = eps.len();
        configure_binding(xhci_port, slot, port, binding, pad, eps);
        if eps.len() > before {
            claim_interface(xhci_port, slot, port, binding.interface_number);
        }
    }
    Outcome::Bound
}
