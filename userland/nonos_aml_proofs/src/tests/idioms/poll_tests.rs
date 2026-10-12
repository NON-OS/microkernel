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

//! A method that loops on a status field until a busy bit clears, the
//! way an SMBus or EC handshake waits (ACPI 6.5, 19.6.153 While, 19.6.125
//! Sleep): bounded by its own counter when the bit clears in time; and
//! when it never clears, stopped by the step budget if the loop spins and
//! by the time budget if it sleeps.

use super::more::*;
use super::run::{ok, run};
use crate::enc::*;
use crate::exec::{ExecLimits, FaultKind};
use crate::probe_regs::Register;
use crate::rig::Rig;

fn rig(clears_after: Option<u32>) -> Rig {
    let busy = bin(AND, &name("STS"), &int(2));
    let bounded = cat(&[
        &store(&int(0), &local(0)),
        &while_(
            &e(LAND, &[&busy, &e(LLESS, &[&local(0), &int(1000)])]),
            &cat(&[&sleep(1), &increment(&local(0))]),
        ),
        &ret(&local(0)),
    ]);
    let defs = cat(&[
        &region("SMB", 1, 0x62, 1),
        &field("SMB", 0x01, &[unit("STS", 8)]),
        &method("\\WAIT", 0, &bounded),
        &method("\\SPIN", 0, &while_(&busy, &[])),
        &method("\\NAP", 0, &while_(&busy, &sleep(1))),
    ]);
    let mut r = Rig::new(&defs).connected();
    r.probe.regs.insert((1, 0x62), Register::Busy { reads: clears_after, busy: 2 });
    r
}

#[test]
fn the_bit_clears_and_the_loop_ends() {
    let mut r = rig(Some(5));
    let (v, ran) = ok(&mut r, "\\WAIT");
    assert_eq!(v, 5, "five busy reads, five sleeps");
    assert_eq!((r.probe.slept_ms.iter().sum::<u64>(), ran.time_us), (5, 5_000));
    assert!(ran.steps < 200, "{} steps", ran.steps);
}

#[test]
fn a_bit_that_never_clears() {
    let mut r = rig(None);
    let (v, _) = ok(&mut r, "\\WAIT");
    assert_eq!(v, 1000, "the firmware's own bound");
    let spin = run(&mut r, "\\SPIN", vec![], ExecLimits { steps: 50_000, ..ExecLimits::DEFAULT });
    assert_eq!(
        (spin.result.map_err(|f| f.kind).err(), spin.steps),
        (Some(FaultKind::Steps), 50_001)
    );
    let nap = run(&mut r, "\\NAP", vec![], ExecLimits { time_ms: 50, ..ExecLimits::DEFAULT });
    assert_eq!(nap.result.map_err(|f| f.kind).err(), Some(FaultKind::Time));
    assert!(nap.time_us <= 50_000, "never past the time budget: {}", nap.time_us);
}
