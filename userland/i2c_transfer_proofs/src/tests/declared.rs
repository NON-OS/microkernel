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

//! The bench's controller as discovery records it, and touchpads the
//! firmware declares on it, on another controller, or on none.

use nonos_i2cmodel::{touchpad, HidOverI2c};

use super::fixture::{MAX_INPUT, PRODUCT, VENDOR};
use crate::bench::controller;
use crate::constants::controller_index;
use crate::discover::{AcpiTouchpad, Found};
use crate::transaction::Reach;

/// The bench's controller, Gemini Lake's I2C0.
pub fn this() -> Found {
    controller()
}

/// A touchpad declared at `addr` (zero: filled in at run time) on I2C`n`.
pub fn on(n: u8, addr: u8) -> AcpiTouchpad {
    AcpiTouchpad { addr, desc_reg: 0x0001, controller_idx: Some(n), ..AcpiTouchpad::default() }
}

/// A touchpad declared at `addr` on the bench's controller.
pub fn here(addr: u8) -> AcpiTouchpad {
    on(controller_index(this().pci_device).expect("I2C0"), addr)
}

/// A touchpad whose ResourceSource names no controller this driver knows.
pub fn nowhere(addr: u8) -> AcpiTouchpad {
    AcpiTouchpad { addr, desc_reg: 0x0001, ..AcpiTouchpad::default() }
}

/// A healthy touchpad answering at `addr`.
pub fn pad_at(addr: u8) -> HidOverI2c {
    HidOverI2c::new(addr, touchpad::report_descriptor(), MAX_INPUT, VENDOR, PRODUCT)
}

pub fn reached(r: &Reach) -> Vec<u8> {
    (0..=u8::MAX).filter(|&a| r.allows(a)).collect()
}
