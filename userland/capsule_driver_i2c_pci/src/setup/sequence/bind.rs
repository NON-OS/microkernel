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

//! Settling the bind on one declared device.

use super::say::say_bound;
use super::settle::settle;
use crate::discover::{AcpiTouchpad, Found};
use crate::driver::Driver;
use crate::setup::doorbell;
use crate::transaction::Presence;

/// Bind `driver`, the controller `dev`, to `tg`, which setup's probe
/// `found` as it says (settle.rs decides the address and the reach).
pub(super) fn bind(
    mut driver: Driver,
    dev: &Found,
    tg: &AcpiTouchpad,
    found: Presence,
    targets: &[AcpiTouchpad],
) -> Driver {
    let settled = settle(&driver, dev, tg, found, targets);
    driver.bound_by_probe = settled.by_probe;
    driver.bound_addr = settled.addr;
    driver.bound_desc_reg = settled.desc_reg;
    (driver.bound_pin, driver.bound_info) = (tg.gpio_pin, tg.info);
    driver.reach.set(settled.reach);
    driver.doorbell = doorbell(driver.device_id, tg);
    say_bound(&driver, Some(tg));
    driver
}
