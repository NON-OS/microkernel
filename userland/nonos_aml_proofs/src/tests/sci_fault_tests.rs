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

//! What an SCI does with a failing method, an event raised while disabled,
//! and the fixed events.

use super::sci_rig::{bits, gpes, SciRig};
use crate::events::{Dispatch, Policy, GBL, PWRBTN, SLPBTN};
use crate::exec::{Fault, FaultKind};
use crate::fadt_enc::{PM1_EN, PM1_STS};

fn fail(_: Dispatch) -> Result<(), Fault> {
    Err(Fault::new(FaultKind::Steps, 0, 0))
}

#[test]
fn a_failing_level_method_leaves_its_event_off_and_an_edge_one_does_not() {
    let aml = gpes();
    let mut rig = SciRig::new(&aml, &[], Policy::default());
    rig.start();
    rig.raise(0x01);
    rig.raise(0x02);
    let mut io = rig.regs.clone();
    let report = rig.events.on_sci(&mut io, &mut fail).unwrap();
    assert_eq!(report.disabled, [0x01]);
    assert_eq!(bits(&rig.regs, 0x01), (false, false));
    assert_eq!(bits(&rig.regs, 0x02), (false, true));
    assert!(!rig.events.armed.contains(&0x01));
    rig.raise(0x01);
    let again = rig.events.on_sci(&mut io, &mut fail).unwrap();
    assert!(again.ran.is_empty(), "a disabled event raises nothing");
}

#[test]
fn unarmed_status_is_left_alone() {
    let aml = gpes();
    let mut rig = SciRig::new(&aml, &[0x0D], Policy::default());
    rig.start();
    rig.raise(0x0D);
    rig.raise(0x07);
    let mut io = rig.regs.clone();
    let report = rig.events.on_sci(&mut io, &mut |_| Ok(())).unwrap();
    assert!(report.ran.is_empty());
    assert_eq!(bits(&rig.regs, 0x0D), (true, false), "for whoever arms it for wake");
}

#[test]
fn fixed_events_are_served_by_policy_and_cleared_one_by_one() {
    let aml = gpes();
    let both = Policy { power_button: true, sleep_button: false };
    let mut rig = SciRig::new(&aml, &[], both);
    rig.start();
    assert_eq!(rig.pm1(PM1_EN), GBL | PWRBTN);
    rig.set_pm1(PM1_STS, GBL | PWRBTN | SLPBTN);
    let mut io = rig.regs.clone();
    let report = rig.events.on_sci(&mut io, &mut |_| Ok(())).unwrap();
    assert_eq!(report.fixed, GBL | PWRBTN);
    assert_eq!(rig.pm1(PM1_STS), SLPBTN, "the sleep button is not the capsule's");
}
