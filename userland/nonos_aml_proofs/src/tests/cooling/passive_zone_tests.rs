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

//! A zone's passive constants and _PSL read through the interpreter.

use crate::consts::at;
use crate::enc::*;
use crate::probe::Probe;
use crate::run::boot;
use crate::tests::platform::run::evaluator;
use crate::thermal::{passive_constants, passive_devices, Constants};

fn zone(tc1: u64) -> Vec<u8> {
    let body = cat(&[
        &name_obj("_PSV", &int(3532)),
        &name_obj("_TC1", &int(tc1)),
        &name_obj("_TC2", &int(5)),
        &name_obj("_TSP", &int(10)),
        &name_obj("_PSL", &package(1, &[name("\\_SB.CPU0")])),
    ]);
    let cpu = cat(&[&name_obj("_HID", &string("ACPI0007")), &name_obj("_UID", &int(0))]);
    cat(&[&thermal_zone("\\_TZ.TZ0", &body), &device("\\_SB.CPU0", &cpu)])
}

#[test]
fn constants_and_processors_come_from_the_zone() {
    let (mut aml, _) = boot(&zone(2), 2);
    let tz = at(&aml, "\\_TZ.TZ0").unwrap();
    let mut probe = Probe::default();
    let c = passive_constants(&mut aml, &mut evaluator(&mut probe), tz);
    assert_eq!(c, Some(Constants { tc1: 2, tc2: 5, period_ms: 1000 }));
    let psl = passive_devices(&mut aml, &mut evaluator(&mut probe), tz);
    assert_eq!(psl, [at(&aml, "\\_SB.CPU0").unwrap()]);
}

#[test]
fn a_lying_constant_leaves_the_zone_without_passive_cooling() {
    let (mut aml, _) = boot(&zone(1000), 2);
    let tz = at(&aml, "\\_TZ.TZ0").unwrap();
    let mut probe = Probe::default();
    assert_eq!(passive_constants(&mut aml, &mut evaluator(&mut probe), tz), None);
}
