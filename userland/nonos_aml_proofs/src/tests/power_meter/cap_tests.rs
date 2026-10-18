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

//! Setting the cap: refused untouched without an enforced, configurable
//! cap or outside _PMC's bounds; written within them; firmware's own
//! refusal reported.

use super::rig::{machine, pmc};
use crate::consts::at;
use crate::power_meter::*;
use crate::probe::Probe;
use crate::run::{boot, call, num};
use crate::tests::platform::run::evaluator;

fn try_cap(caps: u64, configurable: u64, max: u64, mw: u32) -> (Result<(), CapError>, u64) {
    let (mut aml, _) = boot(&machine(&pmc(caps, configurable, max)), 2);
    let dev = at(&aml, "\\_SB.PMT0").unwrap();
    let mut probe = Probe::default();
    let p = read_meter(&mut aml, &mut evaluator(&mut probe), dev).pmc.unwrap();
    let r = set_cap(&mut aml, &mut evaluator(&mut probe), dev, &p, mw);
    (r, num(&call(&mut aml, "\\GSHL", vec![]).unwrap()))
}

#[test]
fn refused_before_firmware_is_asked() {
    assert_eq!(
        try_cap(0b001, 1, 50_000, 20_000),
        (Err(CapError::NotConfigurable), 0),
        "no enforced cap"
    );
    assert_eq!(
        try_cap(0b101, 0, 50_000, 20_000),
        (Err(CapError::NotConfigurable), 0),
        "not settable"
    );
    assert_eq!(try_cap(0b101, 1, 50_000, 9_999), (Err(CapError::OutOfRange), 0), "below");
    assert_eq!(try_cap(0b101, 1, 50_000, 50_001), (Err(CapError::OutOfRange), 0), "above");
}

#[test]
fn written_within_bounds_and_firmware_refusing() {
    assert_eq!(try_cap(0b101, 1, 50_000, 30_000), (Ok(()), 30_000));
    assert_eq!(
        try_cap(0b101, 1, 90_000, 70_000),
        (Err(CapError::OutOfRange), 70_000),
        "firmware said 1"
    );
}
