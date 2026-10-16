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

//! The firmware half of an eject over every path: what each kind of device
//! has (16 combinations) times each step failing. Unlock comes first,
//! `_EJ0` only for a device that has it, and a failure after the unlock
//! always locks again.

use super::release_rig::{Call, Fake};
use super::release_want::want;
use crate::hotplug::{release, Has, Released, Stopped};

#[test]
fn every_kind_of_device_and_every_failing_step() {
    let fails = [None, Some(Call::Lock(false)), Some(Call::Undock), Some(Call::Eject)];
    for bits in 0..16u8 {
        let b = |n: u8| bits & (1 << n) != 0;
        let h = Has { ej0: b(0), removable: b(1), lck: b(2), dock: b(3) };
        for fail in fails {
            let mut f = Fake { calls: Vec::new(), fail };
            let got = release(&mut f, h);
            let (calls, r) = want(h, fail);
            assert_eq!((&f.calls, got), (&calls, r), "{h:?} failing {fail:?}");
            assert!(h.ej0 || !f.calls.contains(&Call::Eject), "no _EJ0 is never run");
            let unlocked =
                f.calls.first() == Some(&Call::Lock(false)) && fail != Some(Call::Lock(false));
            if got.is_err() && unlocked {
                assert_eq!(f.calls.last(), Some(&Call::Lock(true)), "locked again: {h:?}");
            }
        }
    }
}

#[test]
fn a_relock_that_fails_still_reports_the_first_failure() {
    let h = Has { ej0: true, removable: false, lck: true, dock: false };
    let mut f = Fake { calls: Vec::new(), fail: Some(Call::Eject) };
    assert_eq!(release(&mut f, h), Err(Stopped::Eject));
    let mut f = Fake { calls: Vec::new(), fail: Some(Call::Lock(true)) };
    assert_eq!(release(&mut f, h), Ok(Released::ByFirmware), "lock(true) never runs on success");
    assert!(!f.calls.contains(&Call::Lock(true)));
}
