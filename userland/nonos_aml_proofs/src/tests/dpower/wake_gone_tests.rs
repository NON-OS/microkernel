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

//! A holder gone by release or exit, end to end: the capsule's own sweep
//! (`wake_sweep`, by path) picks the devices, newest armed first, and
//! each is disarmed: `_PSW(0)` then `_DSW(0, 0, 0)`, with the event off
//! only with its last user.

use super::rig::{ints, said};
use super::wake_rig::{ids, src, Switches};
use super::wake_sweep::stale;
use crate::device_power::{DState, PowerPool, RuntimeWake};
use crate::probe::Probe;
use crate::run::boot;
use crate::tests::platform::run::evaluator;

/// XHC armed by pid 100 as device 10, then TPD by pid 200 as device 11.
pub fn armed() -> (crate::load::Aml, [u32; 7], Vec<(u32, u32, u64)>) {
    let (aml, _) = boot(&src(), 2);
    let i = ids(&aml);
    (aml, i, vec![(i[0], 100, 10), (i[1], 200, 11)])
}

#[test]
fn release_and_exit_disarm_newest_first_with_the_event_off_last() {
    let (mut aml, [xhc, tpd, .., prw1], owners) = armed();
    let (mut probe, mut pool, mut sw, mut rw) =
        (Probe::default(), PowerPool::default(), Switches::default(), RuntimeWake::default());
    let mut ev = evaluator(&mut probe);
    for d in [xhc, tpd] {
        rw.arm((&mut pool, &mut aml, &mut ev), &mut sw, d, DState::D3Hot).unwrap();
    }
    let gone = stale(&owners, &mut |_, _| None);
    assert_eq!(gone.iter().map(|o| o.0).collect::<Vec<_>>(), [tpd, xhc], "newest first");
    rw.disarm((&mut pool, &mut aml, &mut ev), &mut sw, gone[0].0);
    assert_eq!((sw.log.len(), pool.holders(prw1)), (1, 1), "XHC still uses both");
    rw.disarm((&mut pool, &mut aml, &mut ev), &mut sw, gone[1].0);
    drop(ev);
    assert_eq!((sw.log, pool.holders(prw1)), (vec![(0x6D, true), (0x6D, false)], 0));
    // Armed: on, _DSW(1, 0, 3), _PSW(1). Disarmed: _PSW(0), _DSW(0, 0, 0), off.
    assert_eq!(said(&probe), ints(&[0xC1, 1, 0, 3, 1, 0, 0, 0, 0, 0xC0]));
}

#[test]
fn one_release_leaves_the_other_armed() {
    let (mut aml, [xhc, tpd, .., prw1], owners) = armed();
    let (mut probe, mut pool, mut sw, mut rw) =
        (Probe::default(), PowerPool::default(), Switches::default(), RuntimeWake::default());
    let mut ev = evaluator(&mut probe);
    for d in [xhc, tpd] {
        rw.arm((&mut pool, &mut aml, &mut ev), &mut sw, d, DState::D3Hot).unwrap();
    }
    let gone = stale(&owners, &mut |pid, _| (pid == 200).then_some(tpd));
    assert_eq!(gone, [(xhc, 100, 10)]);
    rw.disarm((&mut pool, &mut aml, &mut ev), &mut sw, xhc);
    assert_eq!((sw.log.len(), pool.holders(prw1), rw.wakes(tpd)), (1, 1, Some(0)));
}
