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

//! A wake event armed while the system runs: enabled and then served by
//! its method like any runtime event, off again when disarmed, and never
//! armed without a method or outside the blocks.

use crate::tests::sci_rig::{bits, gpes, SciRig};
use crate::consts::at;
use crate::events::{Dispatch, EventsError, Policy};

#[test]
fn a_runtime_wake_event_is_armed_served_and_disarmed() {
    let aml = gpes();
    let l0d = at(&aml, "\\_GPE._L0D").unwrap();
    let mut rig = SciRig::new(&aml, &[0x0D], Policy::default());
    rig.start();
    let mut io = rig.regs.clone();
    assert_eq!(bits(&rig.regs, 0x0D), (false, false), "a wake event starts off");
    rig.events.arm_wake(&mut io, 0x0D, true).unwrap();
    assert_eq!(bits(&rig.regs, 0x0D), (false, true));
    rig.raise(0x0D);
    let mut ran = Vec::new();
    let r = rig.events.on_sci(&mut io, &mut |d| {
        ran.push(d);
        Ok(())
    });
    assert!(r.is_ok());
    assert_eq!(ran, [Dispatch::Method(l0d)], "served through firmware's _L0D");
    assert_eq!(bits(&rig.regs, 0x0D), (false, true), "cleared, and on again");
    rig.events.arm_wake(&mut io, 0x0D, false).unwrap();
    assert_eq!(bits(&rig.regs, 0x0D), (false, false));
    assert!(!rig.events.armed.contains(&0x0D));
}

#[test]
fn no_method_or_no_block_is_never_armed() {
    let aml = gpes();
    let mut rig = SciRig::new(&aml, &[], Policy::default());
    rig.start();
    let mut io = rig.regs.clone();
    assert_eq!(rig.events.arm_wake(&mut io, 0x05, true), Err(EventsError::NoMethod));
    assert_eq!(bits(&rig.regs, 0x05), (false, false));
    assert_eq!(rig.events.arm_wake(&mut io, 0x400, true), Err(EventsError::NoBlock));
}
