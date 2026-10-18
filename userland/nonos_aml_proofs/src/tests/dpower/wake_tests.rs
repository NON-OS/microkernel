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

//! Runtime wake: arming in order (resources, `_DSW`/`_PSW`, then the
//! event on its first user), the event shared until its last user lets
//! go, and refusals that change nothing.

use super::rig::{ints, said};
use super::wake_rig::{ids, src, Switches};
use crate::device_power::{DState, PowerPool, RuntimeWake, WakeError};
use crate::probe::Probe;
use crate::run::boot;
use crate::tests::platform::run::evaluator;

#[test]
fn arming_runs_in_order_and_disarming_undoes_it() {
    let (mut aml, _) = boot(&src(), 2);
    let [xhc, tpd, _, _, _, _, prw1] = ids(&aml);
    let (mut probe, mut pool, mut sw, mut rw) =
        (Probe::default(), PowerPool::default(), Switches::default(), RuntimeWake::default());
    let mut ev = evaluator(&mut probe);
    rw.arm((&mut pool, &mut aml, &mut ev), &mut sw, xhc, DState::D3Hot).unwrap();
    rw.arm((&mut pool, &mut aml, &mut ev), &mut sw, tpd, DState::D3Hot).unwrap();
    assert_eq!((pool.holders(prw1), sw.log.clone()), (2, vec![(0x6D, true)]), "one event on");
    assert!(rw.disarm((&mut pool, &mut aml, &mut ev), &mut sw, xhc));
    assert_eq!(sw.log.len(), 1, "TPD still uses the event");
    assert!(rw.disarm((&mut pool, &mut aml, &mut ev), &mut sw, tpd));
    assert!(!rw.disarm((&mut pool, &mut aml, &mut ev), &mut sw, tpd), "not armed twice");
    drop(ev);
    assert_eq!((pool.holders(prw1), sw.log.last()), (0, Some(&(0x6D, false))));
    // XHC: on, _DSW(1, 0, 3). TPD: _PSW(1). Then _DSW(0, 0, 0), _PSW(0), off.
    assert_eq!(said(&probe), ints(&[0xC1, 1, 0, 3, 1, 0, 0, 0, 0, 0xC0]));
}

#[test]
fn refusals_change_nothing() {
    let (mut aml, _) = boot(&src(), 2);
    let [xhc, _, lan, nop, blk, bad, prw1] = ids(&aml);
    let (mut probe, mut pool, mut sw, mut rw) =
        (Probe::default(), PowerPool::default(), Switches::default(), RuntimeWake::default());
    let mut ev = evaluator(&mut probe);
    let mut arm = |d, t| rw.arm((&mut pool, &mut aml, &mut ev), &mut sw, d, t);
    assert_eq!(arm(xhc, DState::D3Cold), Err(WakeError::TooDeep(DState::D3Hot)));
    assert_eq!(arm(nop, DState::D3Hot), Err(WakeError::NoEvent), "no _PRW");
    assert_eq!(arm(blk, DState::D3Hot), Err(WakeError::NoEvent), "a block-form event");
    assert_eq!(arm(bad, DState::D3Hot), Err(WakeError::BadS0w));
    assert_eq!(arm(lan, DState::D3Cold), Ok(()), "LAN wakes from D3cold");
    drop(ev);
    assert_eq!((pool.holders(prw1), sw.log.len(), rw.armed.len()), (1, 1, 1));
}
