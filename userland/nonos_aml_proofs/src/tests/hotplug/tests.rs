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

//! Hot-plug's AML half: which Notify values ask for a check, which devices
//! one covers, and an eject that takes the slot away.

use super::rig::machine;
use crate::consts::at;
use crate::hotplug::*;
use crate::probe::Probe;
use crate::run::boot;
use crate::tests::platform::run::evaluator;

#[test]
fn notify_values_that_ask_for_a_check() {
    let all = [0u8, 1, 2, 3, 4, 0x80].map(check);
    let want = [Some(Check::Bus), Some(Check::Device), None, Some(Check::Eject)];
    assert_eq!(all[..4], want);
    assert_eq!((all[4], all[5]), (Some(Check::DeviceLight), None));
    assert!(Check::Bus.deep() && Check::Device.deep());
    assert!(!Check::DeviceLight.deep() && !Check::Eject.deep());
}

#[test]
fn a_check_covers_the_device_and_what_is_below_it() {
    let (aml, _) = boot(&machine(), 2);
    let ids = |p: &[&str]| p.iter().map(|p| at(&aml, p).unwrap()).collect::<Vec<_>>();
    let rp = at(&aml, "\\_SB.PCI0.RP01").unwrap();
    let below = ids(&["\\_SB.PCI0.RP01", "\\_SB.PCI0.RP01.SLT0", "\\_SB.PCI0.RP01.SLT0.FN1"]);
    assert_eq!(targets(&aml, rp, true), below);
    assert_eq!(targets(&aml, rp, false), [rp]);
}

#[test]
fn an_eject_takes_the_slot_away() {
    let (mut aml, _) = boot(&machine(), 2);
    let mut probe = Probe::default();
    let (slot, rp) =
        (at(&aml, "\\_SB.PCI0.RP01.SLT0").unwrap(), at(&aml, "\\_SB.PCI0.RP01").unwrap());
    assert!(ejectable(&aml, slot) && !ejectable(&aml, rp));
    assert_eq!(present(&mut aml, &mut evaluator(&mut probe), slot), Some(true));
    assert_eq!(eject(&mut aml, &mut evaluator(&mut probe), slot), Ok(()));
    assert_eq!(present(&mut aml, &mut evaluator(&mut probe), slot), Some(false));
    assert_eq!(eject(&mut aml, &mut evaluator(&mut probe), rp), Err(EjectError::NotEjectable));
    assert_eq!(present(&mut aml, &mut evaluator(&mut probe), rp), Some(true), "no _STA");
}
