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

//! The device found by its _HID and read whole; the absent one claiming
//! the id by _CID passed over; the time and an alarm set, and DC's alarm
//! refused by firmware.

use super::rig::machine;
use crate::consts::at;
use crate::platform::inventory;
use crate::probe::Probe;
use crate::run::{boot, call, num};
use crate::tests::platform::run::evaluator;
use crate::time_alarm::*;

#[test]
fn found_and_read() {
    let (mut aml, _) = boot(&machine(), 2);
    let mut probe = Probe::default();
    let (devices, _) = inventory(&mut aml, &mut evaluator(&mut probe));
    let dev = time_alarm_device(&devices).unwrap();
    assert_eq!(dev, at(&aml, "\\_SB.AWAC").unwrap(), "RTC2 is absent");
    let r = read(&mut aml, &mut evaluator(&mut probe), dev);
    assert_eq!(r.capabilities, Some(5));
    assert_eq!(r.now.map(|t| (t.year, t.month, t.day)), Some((2026, 10, 12)));
    let ac = Timer { value: Some(3600), policy: Some(120), wake_status: Some(1) };
    let dc = Timer { value: Some(DISABLED), policy: Some(DISABLED), wake_status: Some(0) };
    assert_eq!((r.ac, r.dc), (ac, dc));
}

#[test]
fn the_writes() {
    let (mut aml, _) = boot(&machine(), 2);
    let mut probe = Probe::default();
    let dev = at(&aml, "\\_SB.AWAC").unwrap();
    let now = read(&mut aml, &mut evaluator(&mut probe), dev).now.unwrap();
    let later = RealTime { hour: 14, ..now };
    assert_eq!(set_time(&mut aml, &mut evaluator(&mut probe), dev, &later), Ok(()));
    let ok = set_alarm(&mut aml, &mut evaluator(&mut probe), dev, AC, 900);
    assert_eq!(ok, Ok(()));
    assert_eq!(num(&call(&mut aml, "\\STVS", vec![]).unwrap()), 900);
    let dc = set_alarm(&mut aml, &mut evaluator(&mut probe), dev, DC, 60);
    assert_eq!(dc, Err(WriteError::Refused(1)));
    let bad = set_alarm(&mut aml, &mut evaluator(&mut probe), dev, 2, 60);
    assert_eq!(bad, Err(WriteError::Invalid));
    let wrong = RealTime { month: 0, ..now };
    let r = set_time(&mut aml, &mut evaluator(&mut probe), dev, &wrong);
    assert_eq!(r, Err(WriteError::Invalid));
}
