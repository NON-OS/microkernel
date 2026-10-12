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

//! Each present meter found by _HID or _CID and read whole; one without
//! methods reads as nothing; a short _PMC is refused.

use super::rig::{machine, pmc};
use crate::consts::at;
use crate::platform::inventory;
use crate::power_meter::*;
use crate::probe::Probe;
use crate::run::boot;
use crate::tests::platform::run::evaluator;

#[test]
fn found_and_read() {
    let (mut aml, _) = boot(&machine(&pmc(0b101, 1, 50_000)), 2);
    let mut probe = Probe::default();
    let (devices, _) = inventory(&mut aml, &mut evaluator(&mut probe));
    let found = meters(&devices);
    let ids = [at(&aml, "\\_SB.PMT0").unwrap(), at(&aml, "\\_SB.PMT1").unwrap()];
    assert_eq!(found, ids, "PMT2 is absent");
    let m = read_meter(&mut aml, &mut evaluator(&mut probe), found[0]);
    assert_eq!((m.power_mw, m.interval_ms, m.cap_mw), (Some(12_500), Some(1000), Some(45_000)));
    let p = m.pmc.expect("_PMC");
    assert_eq!((p.capabilities, p.unit, p.accuracy), (0b101, Some(0), Some(95_000)));
    assert_eq!(
        (p.min_interval_ms, p.max_interval_ms, p.hysteresis_mw),
        (Some(500), Some(5000), Some(100))
    );
    assert!(p.cap_configurable);
    assert_eq!(
        (p.min_cap_mw, p.max_cap_mw, p.model.as_slice()),
        (Some(10_000), Some(50_000), &b"PM-1"[..])
    );
    let bare = read_meter(&mut aml, &mut evaluator(&mut probe), found[1]);
    assert_eq!(bare, Meter::default(), "no methods, nothing read");
}

#[test]
fn unknowns_and_a_short_pmc() {
    let (mut aml, _) = boot(&machine(&pmc(1, 0xFFFF_FFFF, 0xFFFF_FFFF)), 2);
    let dev = at(&aml, "\\_SB.PMT0").unwrap();
    let m = read_meter(&mut aml, &mut evaluator(&mut Probe::default()), dev);
    let p = m.pmc.unwrap();
    assert!(!p.cap_configurable && p.max_cap_mw.is_none());
    let short =
        crate::enc::package(3, &[crate::enc::int(1), crate::enc::int(0), crate::enc::int(0)]);
    let (mut aml, _) = boot(&machine(&short), 2);
    let m = read_meter(&mut aml, &mut evaluator(&mut Probe::default()), dev);
    assert_eq!((m.pmc, m.power_mw), (None, Some(12_500)));
}
