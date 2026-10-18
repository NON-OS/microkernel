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

//! The EC's GPE served natively: cleared before it is served, as an edge
//! event, re-armed even when serving fails, and never armed for wake.

use super::super::sci_rig::{bits, gpes, SciRig};
use crate::events::{Dispatch, Policy};
use crate::exec::{Fault, FaultKind};

#[test]
fn the_ec_gpe_is_served_natively_as_an_edge() {
    let aml = gpes();
    let mut rig = SciRig::new(&aml, &[], Policy::default());
    rig.start();
    let mut io = rig.regs.clone();
    rig.events.arm_native(&mut io, 0x17, false).unwrap();
    assert_eq!(bits(&rig.regs, 0x17), (false, true));
    rig.raise(0x17);
    let watch = rig.regs.clone();
    let mut seen = Vec::new();
    let mut serve = |d| {
        seen.push((d, bits(&watch, 0x17)));
        Err(Fault::new(FaultKind::Region, 0, 0))
    };
    let r = rig.events.on_sci(&mut io, &mut serve).unwrap();
    assert_eq!(seen, [(Dispatch::Native(0x17), (false, false))], "cleared and masked while served");
    assert!(r.disabled.is_empty(), "an edge event is re-armed even after a failure");
    assert_eq!(bits(&rig.regs, 0x17), (false, true));
}

#[test]
fn a_wake_ec_gpe_is_not_armed() {
    let aml = gpes();
    let mut rig = SciRig::new(&aml, &[], Policy::default());
    rig.start();
    let mut io = rig.regs.clone();
    rig.events.arm_native(&mut io, 0x17, true).unwrap();
    assert_eq!(bits(&rig.regs, 0x17), (false, false));
    assert!(rig.events.arm_native(&mut io, 0x60, false).is_err(), "past GPE0's 64 events");
}
