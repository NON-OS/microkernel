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

//! A device that departed: the capsule's own selection (`wake_sweep`, by
//! path) finds its arm by device id, and it is forgotten at once, its
//! event off only with its last user, and nothing run on the device.

use super::rig::{ints, said};
use super::wake_gone_tests::armed;
use super::wake_rig::Switches;
use super::wake_sweep::departed;
use crate::device_power::{DState, PowerPool, RuntimeWake};
use crate::probe::Probe;
use crate::tests::platform::run::evaluator;

#[test]
fn a_departed_device_is_forgotten_with_nothing_run_on_it() {
    let (mut aml, [xhc, tpd, .., prw1], owners) = armed();
    let (mut probe, mut pool, mut sw, mut rw) =
        (Probe::default(), PowerPool::default(), Switches::default(), RuntimeWake::default());
    let mut ev = evaluator(&mut probe);
    for d in [xhc, tpd] {
        rw.arm((&mut pool, &mut aml, &mut ev), &mut sw, d, DState::D3Hot).unwrap();
    }
    for id in [10, 11] {
        let o = departed(&owners, &[id]);
        assert!(rw.forget((&mut pool, &mut aml, &mut ev), &mut sw, o[0].0));
    }
    drop(ev);
    assert_eq!((sw.log, pool.holders(prw1)), (vec![(0x6D, true), (0x6D, false)], 0));
    assert_eq!(said(&probe), ints(&[0xC1, 1, 0, 3, 1, 0xC0]), "no _DSW(0) or _PSW(0)");
}
