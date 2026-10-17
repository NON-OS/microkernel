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

//! Thermal zones read through the interpreter: trip points, the falling
//! _ACx rule, _TMP's believable range, _TZP, and _AL0's fan.

use super::zone_rig::zone;
use crate::consts::at;
use crate::enc::thermal_zone;
use crate::probe::Probe;
use crate::run::boot;
use crate::tests::platform::run::evaluator;
use crate::thermal::{active_devices, temperature, trips, zones};

#[test]
fn trip_points_in_milli_celsius_and_the_fan_that_cools() {
    let (mut aml, _) = boot(&zone(3182), 2);
    let tz = at(&aml, "\\_TZ.TZ0").unwrap();
    assert_eq!(zones(&aml), [tz]);
    let mut probe = Probe::default();
    let t = trips(&mut aml, &mut evaluator(&mut probe), tz, &Default::default());
    assert_eq!((t.critical, t.hot, t.passive), (Some(100_050), Some(90_050), Some(80_050)));
    assert_eq!(t.active, [70_050, 60_050], "_AC2 is warmer than _AC1, so the list stops");
    assert_eq!(t.polling, Some(300));
    assert_eq!(temperature(&mut aml, &mut evaluator(&mut probe), tz), Some(45_050));
    let fans = active_devices(&mut aml, &mut evaluator(&mut probe), tz, 0);
    assert_eq!(fans, [at(&aml, "\\_SB.FAN0").unwrap()]);
    assert!(active_devices(&mut aml, &mut evaluator(&mut probe), tz, 1).is_empty());
}

#[test]
fn unbelievable_readings_are_none() {
    for (tmp, want) in [(0, None), (2231, None), (2232, Some(-49_950)), (4733, None)] {
        let (mut aml, _) = boot(&zone(tmp), 2);
        let tz = at(&aml, "\\_TZ.TZ0").unwrap();
        let mut probe = Probe::default();
        assert_eq!(temperature(&mut aml, &mut evaluator(&mut probe), tz), want, "{tmp}");
    }
}

#[test]
fn a_zone_with_nothing_has_no_trips() {
    let (mut aml, _) = boot(&thermal_zone("\\_TZ.TZ1", &[]), 2);
    let tz = at(&aml, "\\_TZ.TZ1").unwrap();
    let mut probe = Probe::default();
    let t = trips(&mut aml, &mut evaluator(&mut probe), tz, &Default::default());
    assert_eq!(t, Default::default());
    assert_eq!(temperature(&mut aml, &mut evaluator(&mut probe), tz), None);
}
