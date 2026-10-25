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

//! A departed device's resources: the counts go down as for any release,
//! and a resource whose last user it was is turned off only while it is
//! still there. CRD wakes through PRW1 (shared with XHC) and its own PRD,
//! declared below it; when CRD leaves, PRD went with it and its _OFF never
//! runs, while PRW1 is turned off with its last user, XHC.

use super::rig::{ints, said, say};
use super::wake_rig::Switches;
use super::wake_sweep::departed;
use crate::consts::at;
use crate::device_power::{DState, PowerPool, RuntimeWake};
use crate::enc::*;
use crate::probe::Probe;
use crate::run::boot;
use crate::tests::platform::run::evaluator;

fn src() -> Vec<u8> {
    let res = |path: &str, on: u64, off: u64| {
        power_res(path, 0, 0, &cat(&[&method("_ON_", 0, &say(on)), &method("_OFF", 0, &say(off))]))
    };
    let prw = |refs: &[&str]| {
        let mut e = vec![int(0x6D), int(3)];
        e.extend(refs.iter().map(|r| name(r)));
        name_obj("_PRW", &package(e.len() as u8, &e))
    };
    cat(&[
        &res("\\PRW1", 0xC1, 0xC0),
        &device("\\_SB.XHC", &prw(&["\\PRW1"])),
        &device("\\_SB.CRD", &[]),
        &res("\\_SB.CRD.PRD", 0xD1, 0xD0),
        &scope("\\_SB.CRD", &prw(&["\\PRW1", "\\_SB.CRD.PRD"])),
    ])
}

#[test]
fn a_resource_that_went_with_the_device_is_only_forgotten() {
    let (mut aml, _) = boot(&src(), 2);
    let (xhc, crd) = (at(&aml, "\\_SB.XHC").unwrap(), at(&aml, "\\_SB.CRD").unwrap());
    let (prw1, prd) = (at(&aml, "\\PRW1").unwrap(), at(&aml, "\\_SB.CRD.PRD").unwrap());
    let (mut probe, mut pool, mut sw, mut rw) =
        (Probe::default(), PowerPool::default(), Switches::default(), RuntimeWake::default());
    let mut ev = evaluator(&mut probe);
    for d in [xhc, crd] {
        rw.arm((&mut pool, &mut aml, &mut ev), &mut sw, d, DState::D3Hot).unwrap();
    }
    let owners = [(xhc, 100, 10), (crd, 200, 11)];
    let gone = departed(&owners, &[11]);
    assert!(rw.forget((&mut pool, &mut aml, &mut ev), &mut sw, gone[0].0));
    assert_eq!((pool.holders(prw1), pool.holders(prd), sw.log.len()), (1, 0, 1));
    assert!(rw.forget((&mut pool, &mut aml, &mut ev), &mut sw, departed(&owners, &[10])[0].0));
    drop(ev);
    assert_eq!((pool.holders(prw1), sw.log.last()), (0, Some(&(0x6D, false))));
    // On: PRW1, then PRD for CRD. Off: PRW1 alone, with XHC; never PRD.
    assert_eq!(said(&probe), ints(&[0xC1, 0xD1, 0xC0]));
}
