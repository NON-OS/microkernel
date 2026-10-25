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

//! The pins each controller's _AEI lists, and which method each pin's
//! event runs: its own _Exx or _Lxx, else _EVT with the pin; a pin not
//! listed, or a controller with nothing to run, runs nothing.

use super::rig::machine;
use crate::aei::{aei_pins, method_for, signal, AeiError, AeiPin, Ran};
use crate::consts::at;
use crate::probe::Probe;
use crate::run::{boot, call, num};
use crate::tests::platform::run::evaluator;

/// The serial proofs' GpioInt builder writes a debounce of 10 (0.1 ms).
fn pin(pin: u16, edge: bool, polarity: u8, wake: bool) -> AeiPin {
    AeiPin { pin, edge, polarity, shared: false, wake, debounce_centi_ms: 10 }
}

#[test]
fn the_pins_aei_lists() {
    let (mut aml, _) = boot(&machine(), 2);
    let mut probe = Probe::default();
    let gpo0 = at(&aml, "\\_SB.GPO0").unwrap();
    let got = aei_pins(&mut aml, &mut evaluator(&mut probe), gpo0).unwrap();
    let want = [pin(0x12, true, 0, false), pin(0x20, true, 0, false)];
    let want = [&want[..], &[pin(0x40, false, 1, false), pin(0x1A5, false, 1, false)]].concat();
    assert_eq!(got, want, "0x12 again is not listed twice; the GpioIo is no event");
    let gpo2 = at(&aml, "\\_SB.GPO2").unwrap();
    assert_eq!(aei_pins(&mut aml, &mut evaluator(&mut probe), gpo2), None, "no _AEI");
}

#[test]
fn each_pin_runs_its_method() {
    let (mut aml, _) = boot(&machine(), 2);
    let mut probe = Probe::default();
    let gpo0 = at(&aml, "\\_SB.GPO0").unwrap();
    let pins = aei_pins(&mut aml, &mut evaluator(&mut probe), gpo0).unwrap();
    let mut run = |aml: &mut _, p| signal(aml, &mut evaluator(&mut probe), gpo0, &pins, p);
    let ran = |aml: &mut _| num(&call(aml, "\\RAN", vec![]).unwrap());
    let arg = |aml: &mut _| num(&call(aml, "\\ARG", vec![]).unwrap());
    assert_eq!(run(&mut aml, 0x12), Ok(Ran::Edge));
    assert_eq!(ran(&mut aml), 1);
    assert_eq!(run(&mut aml, 0x40), Ok(Ran::Level));
    assert_eq!(ran(&mut aml), 2);
    assert_eq!(run(&mut aml, 0x20), Ok(Ran::Evt), "no _E20 of its own");
    assert_eq!((ran(&mut aml), arg(&mut aml)), (3, 0x20));
    assert_eq!(run(&mut aml, 0x1A5), Ok(Ran::Evt), "past 255");
    assert_eq!(arg(&mut aml), 0x1A5);
    assert_eq!(run(&mut aml, 0x13), Err(AeiError::NotListed));
    assert_eq!(run(&mut aml, 0x50), Err(AeiError::NotListed), "a GpioIo pin");
}

#[test]
fn nothing_to_run() {
    let (mut aml, _) = boot(&machine(), 2);
    let mut probe = Probe::default();
    let gpo1 = at(&aml, "\\_SB.GPO1").unwrap();
    let pins = aei_pins(&mut aml, &mut evaluator(&mut probe), gpo1).unwrap();
    let r = signal(&mut aml, &mut evaluator(&mut probe), gpo1, &pins, 0x300);
    assert_eq!(r, Err(AeiError::NoMethod));
}

#[test]
fn method_names() {
    assert_eq!(method_for(0x12, true), Some(*b"_E12"));
    assert_eq!(method_for(0xAB, false), Some(*b"_LAB"));
    assert_eq!(method_for(0x00, true), Some(*b"_E00"));
    assert_eq!(method_for(0xFF, false), Some(*b"_LFF"));
    assert_eq!(method_for(0x100, true), None);
}
