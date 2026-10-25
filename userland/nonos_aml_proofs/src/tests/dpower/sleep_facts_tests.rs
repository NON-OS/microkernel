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

//! A device's sleep facts, read and never armed: every one from a device
//! that has them all, none from one with none, and each value past its
//! range or from a method that faults read as none.

use crate::consts::at;
use crate::device_power::{sleep_facts, DState, SleepFacts};
use crate::enc::*;
use crate::probe::Probe;
use crate::run::boot;
use crate::tests::platform::run::evaluator;

fn src() -> Vec<u8> {
    let n = |name: &str, v: u64| name_obj(name, &int(v));
    let prw = |sleep: u64| name_obj("_PRW", &package(2, &[int(0x6D), int(sleep)]));
    let divide = e(0x78, &[&int(1), &name("\\ZRO"), &[0], &[0]]);
    let wakes = cat(&[&n("_S0W", 3), &n("_S3W", 3), &n("_S4W", 4), &prw(4)]);
    let full = cat(&[&n("_S3D", 2), &n("_S4D", 3), &wakes]);
    let bad = cat(&[&n("_S3D", 4), &n("_S4W", 9), &prw(7), &method("_S4D", 0, &ret(&divide))]);
    cat(&[
        &n("\\ZRO", 0),
        &device("\\_SB.FULL", &full),
        &device("\\_SB.NONE", &[]),
        &device("\\_SB.BAD", &bad),
    ])
}

#[test]
fn every_fact_none_and_the_ones_past_range() {
    let (mut aml, _) = boot(&src(), 2);
    let mut probe = Probe::default();
    let mut facts = |p: &str| {
        let id = at(&aml, p).unwrap();
        sleep_facts(&mut aml, &mut evaluator(&mut probe), id)
    };
    let full = facts("\\_SB.FULL");
    let want = SleepFacts {
        s0w: Some(DState::D3Hot),
        s3d: Some(DState::D2),
        s4d: Some(DState::D3Hot),
        s3w: Some(DState::D3Hot),
        s4w: Some(DState::D3Cold),
        prw_deepest: Some(4),
    };
    assert_eq!(full, want);
    assert_eq!(full.bytes(), [3, 2, 3, 3, 4, 4]);
    assert_eq!(facts("\\_SB.NONE"), SleepFacts::default());
    assert_eq!(facts("\\_SB.NONE").bytes(), [0xFF; 6]);
    let bad = facts("\\_SB.BAD");
    assert_eq!(bad, SleepFacts::default(), "_S3D 4, _S4W 9, _PRW 7 and a faulting _S4D");
}
