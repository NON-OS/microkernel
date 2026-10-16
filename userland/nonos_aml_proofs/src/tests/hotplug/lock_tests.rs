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

//! `_LCK` runs with the argument asked, `_RMV` counts only when it answers
//! exactly 1, and a device without either says so.

use super::rig::machine;
use crate::consts::at;
use crate::hotplug::{lock, lockable, removable};
use crate::platform::{child, integer};
use crate::probe::Probe;
use crate::run::boot;
use crate::tests::platform::run::evaluator;

#[test]
fn lck_locks_and_unlocks_with_the_argument_asked() {
    let (mut aml, _) = boot(&machine(), 2);
    let mut probe = Probe::default();
    let (dock, slot) =
        (at(&aml, "\\_SB.PCI0.DOCK").unwrap(), at(&aml, "\\_SB.PCI0.RP01.SLT0").unwrap());
    assert!(lockable(&aml, dock) && !lockable(&aml, slot));
    let last = |aml: &mut crate::load::Aml, p: &mut Probe| {
        let r = child(aml, &mut evaluator(p), dock, *b"LKST", Vec::new());
        r.and_then(|r| r.ok()).as_ref().and_then(integer)
    };
    assert_eq!(last(&mut aml, &mut probe), Some(9), "untouched before");
    assert_eq!(lock(&mut aml, &mut evaluator(&mut probe), dock, true), Some(Ok(())));
    assert_eq!(last(&mut aml, &mut probe), Some(1));
    assert_eq!(lock(&mut aml, &mut evaluator(&mut probe), dock, false), Some(Ok(())));
    assert_eq!(last(&mut aml, &mut probe), Some(0));
    assert_eq!(lock(&mut aml, &mut evaluator(&mut probe), slot, true), None, "no _LCK");
}

#[test]
fn rmv_is_removable_only_when_it_answers_one() {
    let (mut aml, _) = boot(&machine(), 2);
    let mut probe = Probe::default();
    let id = |aml: &crate::load::Aml, p: &str| at(aml, p).unwrap();
    let card = id(&aml, "\\_SB.PCI0.CARD");
    let usb = id(&aml, "\\_SB.PCI0.USB0");
    let slot = id(&aml, "\\_SB.PCI0.RP01.SLT0");
    assert!(removable(&mut aml, &mut evaluator(&mut probe), card), "_RMV 1");
    assert!(!removable(&mut aml, &mut evaluator(&mut probe), usb), "_RMV 2 is not 1");
    assert!(!removable(&mut aml, &mut evaluator(&mut probe), slot), "no _RMV");
}
