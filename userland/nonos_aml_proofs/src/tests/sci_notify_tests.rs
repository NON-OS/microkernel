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

//! A GPE method run through the interpreter: its Notify reaches the queue
//! the capsule drains, and a runaway method is stopped by the budget and
//! leaves its level event disabled.

use super::sci_rig::{bits, SciRig};
use crate::consts::at;
use crate::enc::*;
use crate::events::{Dispatch, Policy};
use crate::exec::{ExecLimits, FaultKind, Machine};
use crate::probe::Probe;
use crate::run::boot;

#[test]
fn a_gpe_method_notifies_through_the_interpreter() {
    let notify = e(0x86, &[&name("\\_SB.LID0"), &int(0x80)]);
    let spin = pkg(0xA2, &cat(&[&[0x01], &[0xA3]]));
    let src = cat(&[
        &device("\\_SB.LID0", &[]),
        &method("\\_GPE._L01", 0, &notify),
        &method("\\_GPE._L02", 0, &spin),
    ]);
    let (mut aml, _) = boot(&src, 2);
    let mut rig = SciRig::new(&aml, &[], Policy::default());
    rig.start();
    rig.raise(0x01);
    rig.raise(0x02);
    let mut probe = Probe::default();
    let limits = ExecLimits { steps: 10_000, ..ExecLimits::DEFAULT };
    let mut io = rig.regs.clone();
    let report = rig
        .events
        .on_sci(&mut io, &mut |d| {
            let Dispatch::Method(id) = d else { panic!("no native events here") };
            Machine::new(&mut aml, &mut probe, limits).evaluate(&mut aml, id, vec![]).map(|_| ())
        })
        .unwrap();
    assert_eq!(report.ran[0], (0x01, Ok(())));
    assert_eq!(report.ran[1].1.unwrap_err().kind, FaultKind::Steps);
    assert_eq!(report.disabled, [0x02]);
    assert_eq!(bits(&rig.regs, 0x02), (false, false));
    let lid = at(&aml, "\\_SB.LID0").unwrap();
    assert_eq!(aml.rt.notifications.iter().copied().collect::<Vec<_>>(), [(lid, 0x80)]);
}
