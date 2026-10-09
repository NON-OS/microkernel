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

//! Which ACPI power devices the namespace declares: battery (PNP0C0A), AC
//! adapter (ACPI0003), lid (PNP0C0D), embedded controller (PNP0C09) and a
//! control-method power button (PNP0C0C).
//!
//! Presence is all this can say. Reading a battery means evaluating `_STA`,
//! `_BIX`/`_BIF` and `_BST`, which read the EC through an EmbeddedControl
//! OperationRegion, and that needs an AML interpreter the kernel does not
//! have. Presence is still worth knowing: a desktop declares no battery and
//! can say "no battery" plainly, while a laptop declares one and has to say
//! its status is unavailable rather than show a number it does not have.

use super::scan;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PowerDevices {
    pub battery: bool,
    pub ac_adapter: bool,
    pub lid: bool,
    pub embedded_controller: bool,
    pub power_button: bool,
}

impl PowerDevices {
    pub fn merge(self, o: Self) -> Self {
        Self {
            battery: self.battery || o.battery,
            ac_adapter: self.ac_adapter || o.ac_adapter,
            lid: self.lid || o.lid,
            embedded_controller: self.embedded_controller || o.embedded_controller,
            power_button: self.power_button || o.power_button,
        }
    }
}

fn eisa(hid: &[u8; 8], id: &[u8; 7]) -> bool {
    &hid[..7] == id && (hid[7] == 0 || hid[7] == b' ')
}

fn is_battery(hid: &[u8; 8]) -> bool {
    eisa(hid, b"PNP0C0A")
}

fn is_ac(hid: &[u8; 8]) -> bool {
    hid == b"ACPI0003"
}

fn is_lid(hid: &[u8; 8]) -> bool {
    eisa(hid, b"PNP0C0D")
}

fn is_ec(hid: &[u8; 8]) -> bool {
    eisa(hid, b"PNP0C09")
}

fn is_power_button(hid: &[u8; 8]) -> bool {
    eisa(hid, b"PNP0C0C")
}

/// Classify the devices one AML block declares through `Name (_HID, ...)`.
pub fn classify(aml: &[u8]) -> PowerDevices {
    PowerDevices {
        battery: !scan::find_devices(aml, is_battery).is_empty(),
        ac_adapter: !scan::find_devices(aml, is_ac).is_empty(),
        lid: !scan::find_devices(aml, is_lid).is_empty(),
        embedded_controller: !scan::find_devices(aml, is_ec).is_empty(),
        power_button: !scan::find_devices(aml, is_power_button).is_empty(),
    }
}

/// Classify every block (DSDT and SSDTs).
pub fn classify_blocks<'a, I: IntoIterator<Item = &'a [u8]>>(blocks: I) -> PowerDevices {
    blocks.into_iter().fold(PowerDevices::default(), |acc, b| acc.merge(classify(b)))
}
