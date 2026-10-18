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

//! The first pass over the controllers: probe the declared devices on each.

use nonos_libc::mk_device_release;

use super::bind::bind;
use super::bring_up_one::bring_up_one;
use super::order::{probe_order, standard_mode};
use crate::constants::controller_index;
use crate::discover::{AcpiTouchpad, Found};
use crate::driver::Driver;
use crate::transaction::{probe_hid, Presence};

/// A declared device that acknowledged on the bus its firmware names
/// without returning a descriptor: the controller's index and the device.
pub(super) type Weak = Option<(usize, AcpiTouchpad)>;

/// Bind the controller whose bus returns an HID descriptor from a declared
/// device. Only declared addresses are probed: an address the firmware did
/// not name may be a PMIC or a USB-PD controller, and the probe writes a
/// register pointer. Every controller tried and not kept is released. With
/// no descriptor anywhere, the first device that only acknowledged.
pub(super) fn find_descriptor(cands: &[Found], targets: &[AcpiTouchpad]) -> Result<Driver, Weak> {
    let mut weak: Weak = None;
    if !targets.iter().any(AcpiTouchpad::addressed) {
        return Err(None);
    }
    let order = probe_order(cands, targets);
    for &i in &order.0[..order.1] {
        let dev = cands[i];
        let Ok(driver) = bring_up_one(dev, standard_mode(&dev, targets)) else { continue };
        for tg in targets.iter().filter(|tg| tg.addressed()) {
            match probe_hid(&driver, tg.addr, tg.desc_reg) {
                Ok(found @ Presence::HidDescriptor(_)) => {
                    return Ok(bind(driver, &dev, tg, found, targets))
                }
                Ok(Presence::Acked) if weak.is_none() && tg.names(&dev, controller_index) => {
                    weak = Some((i, *tg));
                }
                _ => {}
            }
        }
        let _ = mk_device_release(driver.device_id);
    }
    Err(weak)
}
