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
use nonos_libc::mk_device_release;

use super::bring_up_one::bring_up_one;
use super::say::{say_bound, say_unreached};
use crate::constants::{controller_index, HID_INFO_TEN_BIT};
use crate::discover::{find_touchpad_addrs, AcpiTouchpad, Found, MAX_CONTROLLERS, MAX_TARGETS};
use crate::driver::Driver;
use crate::setup::gpio::doorbell;
use crate::transaction::{probe_hid, Presence};

/// Addresses HID-over-I2C touchpads ship at, probed only when the firmware
/// declares no address at all (Intel reference firmware patches it in at run
/// time): ELAN, Synaptics/ALPS, and the rest in order of how common they are.
const BLIND_ADDRS: [u8; 8] = [0x15, 0x2C, 0x10, 0x20, 0x24, 0x38, 0x4B, 0x4C];

/// One bring-up attempt over the controllers discovery found. Every
/// controller tried and not kept is released (by `bring_up_one` when it
/// fails, here when its bus does not answer), so the next attempt can claim
/// them again.
pub fn run(cands: &[Found]) -> Result<Driver, &'static str> {
    if cands.is_empty() {
        return Err("i2c-pci: controller not found");
    }
    let mut buf = [AcpiTouchpad::default(); MAX_TARGETS];
    let t = find_touchpad_addrs(&mut buf);
    let targets: &[AcpiTouchpad] = &buf[..t];
    let order = probe_order(cands, targets);
    let order = &order.0[..order.1];

    // Pass 1: bind the controller whose bus returns an HID descriptor from a
    // declared device; remember the first one where a device only ACKed.
    let mut weak: Option<(usize, AcpiTouchpad, u16)> = None;
    for &i in order {
        let dev = cands[i];
        let Ok(mut driver) = bring_up_one(dev, standard_mode(&dev, targets)) else { continue };
        let mut hit = None;
        for tg in targets.iter().filter(|tg| usable(tg)) {
            match probe_hid(&driver, tg.addr, tg.desc_reg) {
                Ok(Presence::HidDescriptor(reg)) => {
                    hit = Some((*tg, reg));
                    break;
                }
                Ok(Presence::Acked) if weak.is_none() && tg.names(&dev, controller_index) => {
                    weak = Some((i, *tg, tg.desc_reg));
                }
                _ => {}
            }
        }
        if let Some((tg, reg)) = hit {
            return Ok(bind(driver, &tg, reg, true));
        }
        if targets.is_empty() {
            if let Some((addr, reg)) = blind_scan(&driver) {
                driver.bound_by_probe = true;
                driver.bound_addr = addr;
                driver.bound_desc_reg = reg;
                say_bound(&driver, None);
                return Ok(driver);
            }
        }
        let _ = mk_device_release(driver.device_id);
    }
    // Pass 2: no descriptor anywhere, which this early can mean the pad is
    // still asleep. A device that acknowledged on its firmware-named bus, then
    // the firmware-named bus alone, beat a blind first fit; the HID driver
    // keeps reprobing until the device wakes.
    if let Some((i, tg, reg)) = weak {
        if let Ok(driver) = bring_up_one(cands[i], standard_mode(&cands[i], targets)) {
            return Ok(bind(driver, &tg, reg, true));
        }
    }
    for tg in targets {
        for dev in cands.iter().filter(|d| tg.names(d, controller_index)) {
            if let Ok(driver) = bring_up_one(*dev, standard_mode(dev, targets)) {
                return Ok(bind(driver, tg, tg.desc_reg, false));
            }
        }
    }
    if let Some(tg) = targets.first() {
        say_unreached(tg, cands);
    }
    // Last resort so a single-controller box still works.
    for dev in cands {
        if let Ok(driver) = bring_up_one(*dev, standard_mode(dev, targets)) {
            return Ok(driver);
        }
    }
    Err("i2c-pci: no controller answered")
}

/// A declared device the probe can address: a 7-bit address is known.
fn usable(tg: &AcpiTouchpad) -> bool {
    tg.addr != 0 && tg.info & HID_INFO_TEN_BIT == 0
}

fn bind(mut driver: Driver, tg: &AcpiTouchpad, reg: u16, by_probe: bool) -> Driver {
    driver.bound_by_probe = by_probe;
    driver.bound_addr = tg.addr;
    driver.bound_desc_reg = reg;
    driver.doorbell = doorbell(driver.pci_device, tg);
    say_bound(&driver, Some(tg));
    driver
}

/// Controllers in probe order: those a declared device names first (in
/// declaration order), then the rest in discovery order.
fn probe_order(cands: &[Found], targets: &[AcpiTouchpad]) -> ([usize; MAX_CONTROLLERS], usize) {
    let mut out = [0usize; MAX_CONTROLLERS];
    let mut n = 0usize;
    let named = |i: usize| targets.iter().any(|tg| tg.names(&cands[i], controller_index));
    for pass in [true, false] {
        for i in 0..cands.len().min(MAX_CONTROLLERS) {
            if named(i) == pass && !out[..n].contains(&i) {
                out[n] = i;
                n += 1;
            }
        }
    }
    (out, n)
}

/// Standard mode when a device this controller may carry declared a speed
/// below 400 kHz: one it is named by, or one that names no controller.
fn standard_mode(dev: &Found, targets: &[AcpiTouchpad]) -> bool {
    targets.iter().any(|tg| {
        let unnamed = tg.controller_idx.is_none() && tg.host_base == 0;
        (unnamed || tg.names(dev, controller_index)) && tg.wants_standard_mode()
    })
}

/// With no address from the firmware, look for an HID descriptor at the
/// addresses touchpads ship at.
fn blind_scan(driver: &Driver) -> Option<(u8, u16)> {
    BLIND_ADDRS.iter().find_map(|&addr| match probe_hid(driver, addr, 0x0001) {
        Ok(Presence::HidDescriptor(reg)) => Some((addr, reg)),
        _ => None,
    })
}
