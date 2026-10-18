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

//! The router over the AMD model: each pending pin delivered to its owner
//! alone (a device's holder, or capsule_acpi for an `_AEI` pin), a pin
//! bound to one owner refused to another, and an output driven only for
//! the holder of the device whose GpioIo names it.

use super::amd_model::bank;
use nonos_gpio::amd_bits::OUTPUT_ENABLE;
use nonos_gpio::{BindError, Delivery, OutputError, Owner, Polarity, Router, Trigger};

const EDGE: Trigger = Trigger { edge: true, polarity: Polarity::Low };

#[test]
fn each_pin_to_its_owner() {
    let (mut b, mut r) = (bank(), Router::default());
    r.bind(&mut b, 3, Owner::Device(7), EDGE).unwrap();
    r.bind(&mut b, 4, Owner::Aei, EDGE).unwrap();
    assert_eq!(r.bind(&mut b, 3, Owner::Device(8), EDGE), Err(BindError::Taken));
    assert_eq!(r.bind(&mut b, 3, Owner::Device(7), EDGE), Ok(()), "again by its owner");
    b.window.raise(3);
    b.window.raise(4);
    b.window.raise(5);
    let got = r.interrupt(&mut b, 0);
    assert_eq!(got, [Delivery::Holder { device: 7, pin: 3 }, Delivery::Acpi { pin: 4 }]);
    assert!(r.interrupt(&mut b, 1).is_empty(), "pin 5 is no one's and never read");
}

#[test]
fn outputs_only_for_their_holder() {
    let (mut b, mut r) = (bank(), Router::default());
    b.window.regs.insert(20 * 4, OUTPUT_ENABLE);
    r.bind_output(20, 9).unwrap();
    assert_eq!(r.bind_output(20, 10), Err(BindError::Taken));
    assert_eq!(r.output(&mut b, |d| d == 10, 20, true), Err(OutputError::NotYours));
    assert_eq!(r.output(&mut b, |d| d == 9, 21, true), Err(OutputError::NotYours), "no GpioIo");
    r.output(&mut b, |d| d == 9, 20, true).unwrap();
    assert_ne!(b.window.pin(20) & nonos_gpio::amd_bits::OUTPUT_VALUE, 0);
}
