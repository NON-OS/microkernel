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

//! The control-method buttons and lids among the devices the inventory
//! found (ACPI 6.5, 4.8.2.2 and 9.4).

use super::state::Buttons;
use crate::sys::log::Line;
use nonos_aml::button::{button_of, Button};
use nonos_aml::platform::{DeviceInfo, PRESENT};

pub fn find(devices: &[DeviceInfo]) -> Buttons {
    let mut b = Buttons::default();
    for d in devices.iter().filter(|d| d.sta & PRESENT != 0) {
        match button_of(d) {
            Some(Button::Power) => b.power.push(d.id),
            Some(Button::Sleep) => b.sleep.push(d.id),
            Some(Button::Lid) => b.lids.push(d.id),
            None => {}
        }
    }
    Line::new(b"buttons power=")
        .dec(b.power.len() as u64)
        .str(b" sleep=")
        .dec(b.sleep.len() as u64)
        .str(b" lid=")
        .dec(b.lids.len() as u64)
        .end();
    b
}
