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

//! The bench's controller bound as setup binds one (setup/sequence/bind.rs):
//! to a declared touchpad as setup's probe found it, or, for a declaration
//! whose address firmware fills in at run time, to the first device that
//! answered the sweep (setup/sequence/sweep.rs).

use super::bound::driver;
use super::controller::controller;
use crate::discover::{AcpiTouchpad, Found};
use crate::driver::Driver;
use crate::setup::settle::settle;
use crate::transaction::Presence;

/// Bound to `tg`, one of the declarations `targets`, which the probe
/// `found` as it says.
pub fn settled(
    clock_hz: u32,
    tg: &AcpiTouchpad,
    found: Presence,
    targets: &[AcpiTouchpad],
) -> Result<Driver, &'static str> {
    settled_on(&controller(), clock_hz, tg, found, targets)
}

/// The same, bound as the controller `dev` discovery listed, the bench's
/// core standing in for that controller's own.
pub fn settled_on(
    dev: &Found,
    clock_hz: u32,
    tg: &AcpiTouchpad,
    found: Presence,
    targets: &[AcpiTouchpad],
) -> Result<Driver, &'static str> {
    let mut d = driver(clock_hz, 0)?;
    (d.device_id, d.pci_device) = (dev.device_id, dev.pci_device);
    let s = settle(&d, dev, tg, found, targets);
    (d.bound_addr, d.bound_desc_reg, d.bound_by_probe) = (s.addr, s.desc_reg, s.by_probe);
    d.reach.set(s.reach);
    Ok(d)
}

/// Bound to the pad of a declaration with no address on this controller.
pub fn unaddressed(clock_hz: u32) -> Result<Driver, &'static str> {
    let tpd0 = AcpiTouchpad {
        desc_reg: 0x0001,
        host: Some(controller().device_id),
        ..AcpiTouchpad::default()
    };
    settled(clock_hz, &tpd0, Presence::Absent, &[tpd0])
}
