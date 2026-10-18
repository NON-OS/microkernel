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

//! A multi-SKU chassis on the bench: three pads declared on the controller,
//! the second one fitted, and the controller bound the way setup binds it.
//! The fitted pad sits outside the HID driver's blind scan, at 0x2A with
//! its descriptor at 0x0030, so only being named finds it.

use nonos_i2cmodel::{Bus, Config, Designware};

use super::acpi::named_once;
use super::declared::{here, pad_at};
use crate::bench::{settled, GEMINI_LAKE_HZ};
use crate::discover::AcpiTouchpad;
use crate::driver::Driver;
use crate::regs::{self, Attached};
use crate::transaction::Presence;

pub const PADS: [u8; 3] = [0x15, 0x2A, 0x10];
pub const FITTED: u8 = PADS[1];
/// The fitted pad's HID descriptor register, as its declaration names it.
pub const FITTED_REG: u16 = 0x0030;

/// The three declarations, the fitted pad's naming its own register.
pub fn declared() -> [AcpiTouchpad; 3] {
    let mut pads = PADS.map(here);
    pads[1].desc_reg = FITTED_REG;
    pads
}

/// The addresses `times` asks of the shipping OP_ACPI_HID handler name.
pub fn named(d: &Driver, times: usize) -> Vec<u8> {
    (0..times).filter_map(|_| named_once(d)).map(|(addr, ..)| addr).collect()
}

/// Bound once setup's probe `found` what it says, with only the fitted pad
/// on the bus: to the fitted pad when it returned its descriptor, else to
/// the first declaration.
pub fn bound(found: Presence) -> (Driver, Attached) {
    let fitted = pad_at(FITTED).descriptor_at(FITTED_REG);
    let a = regs::attach(Designware::new(Config::LPSS, Bus::default().with(fitted)));
    let pads = declared();
    let tg = if matches!(found, Presence::HidDescriptor(_)) { pads[1] } else { pads[0] };
    (settled(GEMINI_LAKE_HZ, &tg, found, &pads).expect("bind"), a)
}
