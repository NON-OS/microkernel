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

//! One SCI end to end: armed GPEs run their methods with the event disabled,
//! edge status cleared before the method and level status after.

use super::sci_rig::{bits, gpes, SciRig};
use crate::consts::at;
use crate::events::{Dispatch, Policy, GBL, PWRBTN};
use crate::fadt_enc::{PM1_EN, PM1_STS};

#[test]
fn start_arms_runtime_events_and_never_a_wake_or_unblocked_one() {
    let aml = gpes();
    let mut rig = SciRig::new(&aml, &[0x0D], Policy::default());
    rig.raise(0x01);
    rig.set_pm1(PM1_STS, PWRBTN | GBL);
    rig.start();
    assert_eq!(bits(&rig.regs, 0x01), (false, true), "stale status cleared, armed");
    assert_eq!(bits(&rig.regs, 0x02), (false, true));
    assert_eq!(bits(&rig.regs, 0x0D), (false, false), "a wake GPE");
    assert_eq!(rig.pm1(PM1_EN), GBL, "the lock's release, no buttons");
    assert_eq!(rig.pm1(PM1_STS), PWRBTN, "the kernel's press is left for it");
    assert_eq!(rig.events.armed.iter().copied().collect::<Vec<_>>(), [0x01, 0x02]);
}

#[test]
fn level_status_clears_after_the_method_and_edge_status_before() {
    let aml = gpes();
    let (l01, e02) = (at(&aml, "\\_GPE._L01").unwrap(), at(&aml, "\\_GPE._E02").unwrap());
    let mut rig = SciRig::new(&aml, &[], Policy::default());
    rig.start();
    rig.raise(0x01);
    rig.raise(0x02);
    let watch = rig.regs.clone();
    let mut seen = Vec::new();
    let mut method = |d| {
        let Dispatch::Method(id) = d else { panic!("no native events here") };
        let gpe = if id == l01 { 0x01 } else { 0x02 };
        seen.push((id, bits(&watch, gpe)));
        Ok(())
    };
    let mut io = rig.regs.clone();
    let report = rig.events.on_sci(&mut io, &mut method).unwrap();
    // While each method runs its event is disabled; the level one's status
    // is still set (the source is still asserting), the edge one's is not.
    assert_eq!(seen, [(l01, (true, false)), (e02, (false, false))]);
    assert_eq!(report.ran, [(0x01, Ok(())), (0x02, Ok(()))]);
    assert_eq!(bits(&rig.regs, 0x01), (false, true));
    assert_eq!(bits(&rig.regs, 0x02), (false, true));
}
