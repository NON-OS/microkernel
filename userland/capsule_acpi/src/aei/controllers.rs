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

//! The controllers asked about so far, each with the pins its _AEI listed
//! when first asked and its event rate. _AEI is read once: the pins a
//! controller signals are fixed by the board.

use super::rate::Rate;
use crate::boot::{evaluator, Acpi};
use alloc::vec::Vec;
use nonos_aml::aei::{aei_pins, AeiPin};
use nonos_aml::namespace::Id;

pub struct Controller {
    pub id: Id,
    pub pins: Vec<AeiPin>,
    pub rate: Rate,
}

#[derive(Default)]
pub struct GpioEvents {
    pub controllers: Vec<Controller>,
}

/// The controller's entry, made on first use; None when it has no _AEI.
pub fn controller(acpi: &mut Acpi, id: Id) -> Option<usize> {
    if let Some(i) = acpi.aei.controllers.iter().position(|c| c.id == id) {
        return Some(i);
    }
    let pins = aei_pins(&mut acpi.aml, &mut evaluator(&mut acpi.host), id)?;
    acpi.aei.controllers.push(Controller { id, pins, rate: Rate::default() });
    Some(acpi.aei.controllers.len() - 1)
}
