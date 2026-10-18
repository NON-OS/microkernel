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
//! A board's other devices beside the touchpad, declared with no address:
//! a GPIO expander at 0x20, whose interrupt latch a read clears, and a
//! thermal sensor at 0x4C, whose configuration register the HID driver's
//! register pointer, 01 00, would write. The controller's sweep tries only
//! 0x15, 0x2C and 0x10, so neither is bound, and the HID driver, which
//! reaches only what the controller put in reach, never addresses either:
//! not with the pad found, nor silent, nor where the sweep does not look.

use i2c_transfer_proofs::bench::{unaddressed, GEMINI_LAKE_HZ};
use i2c_transfer_proofs::regs;
use nonos_i2cmodel::{touchpad, Bus, BusEvent, Config, Designware, Expander, HidOverI2c};
use nonos_i2cmodel::{Sensor, Shared};
use nonos_libc::serve;

use super::fixture::{pad, MAX_INPUT};
use super::service::service;
use crate::setup;

const EXPANDER: u8 = 0x20;
const SENSOR: u8 = 0x4C;

/// Whether the HID driver found `device` beside the two; it fails the test
/// if either was addressed, read or written.
fn found_beside_them(device: HidOverI2c) -> bool {
    let (expander, sensor) =
        (Shared::new(Expander::new(EXPANDER)), Shared::new(Sensor::new(SENSOR)));
    let bus = Bus::default().with(device).with(expander.clone()).with(sensor.clone());
    let _attached = regs::attach(Designware::new(Config::LPSS, bus));
    let controller = unaddressed(GEMINI_LAKE_HZ).expect("controller bring-up");
    let _served = serve(move |req, resp| service(&controller, req, resp));
    let found = setup::run().expect("setup").found();
    let named = |e: &BusEvent| match *e {
        BusEvent::Start { addr, .. } | BusEvent::Restart { addr, .. } => {
            [EXPANDER, SENSOR].contains(&addr)
        }
        _ => false,
    };
    assert!(!regs::with(|c| c.bus().trace().iter().any(named)), "a bystander was addressed");
    assert_eq!((expander.lock().reads(), sensor.lock().reads()), (0, 0), "a bystander was read");
    assert!(expander.lock().writes().is_empty(), "the HID driver wrote to the expander");
    assert!(sensor.lock().writes().is_empty(), "the HID driver wrote to the sensor");
    found
}

fn pad_at(addr: u8) -> HidOverI2c {
    HidOverI2c::new(addr, touchpad::report_descriptor(), MAX_INPUT, 0x04F3, 0x3028)
}

#[test]
fn the_pad_at_0x10_or_0x2c_is_found_and_neither_bystander_is_touched() {
    for at in [0x10, 0x2C] {
        assert!(found_beside_them(pad_at(at)), "the pad at {at:#04x} was not found");
    }
}

#[test]
fn a_silent_pad_or_one_the_sweep_does_not_try_leaves_them_untouched() {
    assert!(!found_beside_them(pad().deaf()), "a pad that does not answer was bound");
    assert!(!found_beside_them(pad_at(0x4B)), "a pad off the sweep was bound");
}
