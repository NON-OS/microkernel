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

//! Notify on a bus device as an event handler issues it (ACPI 6.5, 5.6.6:
//! Bus Check 0 on the bridge, Device Check 1 on a slot), queued in order
//! for the capsule; and a handler that notifies more than the queue holds,
//! whose excess is counted, not kept, and does not stop the method.

use super::more::*;
use super::run::{run, Ran};
use crate::enc::*;
use crate::exec::ExecLimits;
use crate::rig::Rig;

fn rig(storm: usize) -> Rig {
    let check = cat(&[&notify("\\_SB.PCI0", 0), &notify("\\_SB.PCI0.RP01", 1)]);
    let many: Vec<u8> = (0..storm).flat_map(|_| notify("\\_SB.PCI0.RP01", 1)).collect();
    let defs = cat(&[
        &device("\\_SB.PCI0", &name_obj("_HID", &int(0x030A_D041))),
        &device("\\_SB.PCI0.RP01", &name_obj("_ADR", &int(0x001C_0000))),
        &method("\\_GPE._L01", 0, &check),
        &method("\\_GPE._L02", 0, &cat(&[&many, &ret(&int(1))])),
    ]);
    Rig::new(&defs).connected()
}

fn go(r: &mut Rig, path: &str) -> Ran {
    run(r, path, vec![], ExecLimits::DEFAULT)
}

#[test]
fn bus_and_device_checks_queue_in_order() {
    let mut r = rig(0);
    assert!(go(&mut r, "\\_GPE._L01").result.is_ok());
    let at = |p| crate::consts::at(&r.aml, p).unwrap();
    let q: Vec<_> = r.aml.rt.notifications.iter().copied().collect();
    assert_eq!(q, [(at("\\_SB.PCI0"), 0), (at("\\_SB.PCI0.RP01"), 1)]);
}

#[test]
fn more_than_the_queue_holds_is_counted_and_the_method_ends() {
    let cap = crate::exec::Runtime::MAX_NOTIFICATIONS;
    let mut r = rig(cap + 36);
    let ran = go(&mut r, "\\_GPE._L02");
    assert_eq!(ran.result.map(|v| crate::run::num(&v)), Ok(1));
    assert_eq!((r.aml.rt.notifications.len(), r.aml.rt.dropped_notifications), (cap, 36));
}
