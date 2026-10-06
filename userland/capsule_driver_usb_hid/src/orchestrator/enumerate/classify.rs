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
//! interfaces, or bringing it up as a hub.

use alloc::vec::Vec;

use crate::descriptors::{configuration_value, hid_bindings, is_hub};
use crate::hub::Place;
use crate::xhci::{control_transfer, MAX_DESCRIPTOR_LEN};

use super::super::binding::configure_binding;
use super::attach_hub::attach_hub;
use super::devices::Devices;
use super::read_configuration::read_configuration;
use super::types::Outcome;

const SET_CONFIGURATION: u8 = 0x09;

pub(super) fn classify(xhci_port: u32, slot: u8, place: Place, devs: &mut Devices) -> Outcome {
    let mut desc = [0u8; MAX_DESCRIPTOR_LEN];
    let Some(len) = read_configuration(xhci_port, slot, &mut desc) else {
        return Outcome::Failed;
    };
    let raw = &desc[..len];
    let hub = is_hub(raw);
    let bindings = match hid_bindings(raw) {
        _ if hub => Vec::new(),
        Ok(b) if !b.is_empty() => b,
        _ => return Outcome::NotHid,
    };
    let value = configuration_value(raw).unwrap_or(1) as u16;
    let mut dummy = [0u8; 0];
    if control_transfer(xhci_port, slot, 0x00, SET_CONFIGURATION, value, 0, 0, &mut dummy).is_err()
    {
        return Outcome::Failed;
    }
    if hub {
        return attach_hub(xhci_port, slot, place, raw, devs);
    }
    for binding in bindings {
        configure_binding(xhci_port, slot, place.root_port, binding, &mut devs.eps);
    }
    Outcome::Bound
}
