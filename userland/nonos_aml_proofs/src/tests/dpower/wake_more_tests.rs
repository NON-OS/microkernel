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

//! Runtime wake's edges: no `_S0W` means D3hot at most, a failed switch
//! is undone, wakes count only while armed, and re-arming starts afresh.

use super::wake_rig::{ids, src, Switches};
use crate::device_power::{DState, PowerPool, RuntimeWake, WakeError};
use crate::probe::Probe;
use crate::run::boot;
use crate::tests::platform::run::evaluator;

#[test]
fn no_s0w_means_d3hot_at_most() {
    let (mut aml, _) = boot(&src(), 2);
    let [_, tpd, ..] = ids(&aml);
    let (mut probe, mut pool, mut sw, mut rw) =
        (Probe::default(), PowerPool::default(), Switches::default(), RuntimeWake::default());
    let mut ev = evaluator(&mut probe);
    let r = rw.arm((&mut pool, &mut aml, &mut ev), &mut sw, tpd, DState::D3Cold);
    assert_eq!(r, Err(WakeError::TooDeep(DState::D3Hot)));
}

#[test]
fn a_failed_switch_is_undone_and_wakes_count_only_while_armed() {
    let (mut aml, _) = boot(&src(), 2);
    let [xhc, tpd, _, _, _, _, prw1] = ids(&aml);
    let (mut probe, mut pool, mut rw) = (Probe::default(), PowerPool::default(), RuntimeWake::default());
    let mut ev = evaluator(&mut probe);
    let mut bad = Switches { fail: true, ..Switches::default() };
    let r = rw.arm((&mut pool, &mut aml, &mut ev), &mut bad, xhc, DState::D3Hot);
    assert_eq!((r, pool.holders(prw1), rw.armed.len()), (Err(WakeError::Event), 0, 0));
    let mut sw = Switches::default();
    rw.arm((&mut pool, &mut aml, &mut ev), &mut sw, xhc, DState::D3Hot).unwrap();
    assert!(rw.woken(xhc) && rw.woken(xhc) && !rw.woken(tpd));
    assert_eq!((rw.wakes(xhc), rw.wakes(tpd)), (Some(2), None));
    rw.arm((&mut pool, &mut aml, &mut ev), &mut sw, xhc, DState::D2).unwrap();
    assert_eq!((rw.wakes(xhc), pool.holders(prw1)), (Some(0), 1), "re-armed afresh, held once");
}
