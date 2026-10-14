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

//! What a bind settles on: the address and descriptor register OP_ACPI_HID
//! answers with, and the addresses a client may reach (reach.rs in
//! transaction/types says why that set is kept small). A device that
//! returned its HID descriptor to setup's probe is alone in reach. A bind
//! made before any did (multi-SKU firmware declares every pad a chassis may
//! carry, and the fitted one may still be asleep) keeps in reach each pad
//! declared on this controller or on none, and the one device the sweep
//! found for a declaration with no address (sweep.rs); the first time a
//! client reads a valid HID descriptor from one of them through
//! OP_TRANSFER, at its declared register, the reach narrows to that address
//! for the rest of the session (narrow.rs in transaction/types). An address
//! declared twice carries the bound declaration's register, the one
//! OP_ACPI_HID answers.

use core::iter::once;

use super::sweep::sweep;
use crate::constants::controller_index;
use crate::discover::{AcpiTouchpad, Found};
use crate::driver::Driver;
use crate::transaction::{Presence, Reach};

pub struct Settled {
    pub addr: u8,
    pub desc_reg: u16,
    /// The device answered setup's probe.
    pub by_probe: bool,
    pub reach: Reach,
}

/// Settle a bind of `driver`, the controller `dev`, to the declared device
/// `tg`, which setup's probe `found` as it says. With no address declared,
/// the device the sweep found is the one answered.
pub fn settle(
    driver: &Driver,
    dev: &Found,
    tg: &AcpiTouchpad,
    found: Presence,
    targets: &[AcpiTouchpad],
) -> Settled {
    if let Presence::HidDescriptor(reg) = found {
        return Settled {
            addr: tg.addr,
            desc_reg: reg,
            by_probe: true,
            reach: Reach::declared([(tg.addr, reg)]),
        };
    }
    let swept = sweep(driver, dev, targets);
    let addr = if tg.addressed() { tg.addr } else { swept.map_or(0, |(a, _)| a) };
    let here = targets.iter().filter(|t| t.names_none() || t.names(dev, controller_index));
    let declared = here.filter(|t| t.addressed()).map(|t| (t.addr, t.desc_reg));
    Settled {
        addr,
        desc_reg: tg.desc_reg,
        by_probe: found == Presence::Acked || (!tg.addressed() && addr != 0),
        reach: Reach::declared(once((addr, tg.desc_reg)).chain(declared).chain(swept)),
    }
}
