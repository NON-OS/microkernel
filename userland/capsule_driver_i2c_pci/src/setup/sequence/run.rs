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

use super::bind::bind;
use super::bring_up_one::bring_up_one;
use super::order::{declared, standard_mode};
use super::probe_pass::find_descriptor;
use super::say::say_unreached;
use crate::discover::{find_touchpad_addrs, AcpiTouchpad, Found, MAX_TARGETS};
use crate::driver::Driver;
use crate::transaction::Presence;

/// One bring-up attempt over the controllers discovery found. Every
/// controller tried and not kept is released (by `bring_up_one` when it
/// fails, by the first pass when its bus does not answer), so the next
/// attempt can claim them again. An address the firmware did not declare is
/// only ever read, one byte, on a controller a declaration with no address
/// names (sweep.rs), and is in a client's reach only if it answered there.
pub fn run(cands: &[Found]) -> Result<Driver, &'static str> {
    if cands.is_empty() {
        return Err("i2c-pci: controller not found");
    }
    let mut buf = [AcpiTouchpad::default(); MAX_TARGETS];
    let t = find_touchpad_addrs(&mut buf);
    let targets: &[AcpiTouchpad] = &buf[..t];
    let weak = match find_descriptor(cands, targets) {
        Ok(driver) => return Ok(driver),
        Err(weak) => weak,
    };
    // No descriptor anywhere, which this early can mean the pad is still
    // asleep. A device that acknowledged on its firmware-named bus, then the
    // firmware-named bus alone, beat a blind first fit; the HID driver keeps
    // reprobing until the device wakes.
    if let Some((i, tg)) = weak {
        if let Ok(driver) = bring_up_one(cands[i], standard_mode(&cands[i], targets)) {
            return Ok(bind(driver, &cands[i], &tg, Presence::Acked, targets));
        }
    }
    for tg in targets {
        for dev in cands.iter().filter(|d| tg.names(d)) {
            if let Ok(driver) = bring_up_one(*dev, standard_mode(dev, targets)) {
                return Ok(bind(driver, dev, tg, Presence::Absent, targets));
            }
        }
    }
    if let Some(tg) = targets.first() {
        say_unreached(tg, cands);
    }
    // Last resort so a single-controller box still works, with the declared
    // addresses that may sit on it, and only those, in reach.
    for dev in cands {
        if let Ok(driver) = bring_up_one(*dev, standard_mode(dev, targets)) {
            driver.reach.set(declared(dev, targets));
            return Ok(driver);
        }
    }
    Err("i2c-pci: no controller answered")
}
