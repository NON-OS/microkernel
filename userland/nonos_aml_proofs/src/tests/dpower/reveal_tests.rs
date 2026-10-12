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

//! Devices enumeration missed: one that answers after _PS0 and stays on,
//! one that never answers and goes straight back down, and one left
//! unclaimed at the end of boot and powered down then.

use super::rig::{ids, ints, said, src};
use crate::device_power::{reveal, settle, DState, DevicePower, PowerPool, Revealed};
use crate::probe::Probe;
use crate::run::boot;
use crate::tests::platform::run::evaluator;

#[test]
fn answers_after_ps0_and_stays_until_unclaimed() {
    let (mut aml, _) = boot(&src(), 2);
    let [d1, d2, _, _, pra, prb] = ids(&aml);
    let (mut probe, mut dp, mut pool) =
        (Probe::default(), DevicePower::default(), PowerPool::default());
    let mut ev = evaluator(&mut probe);
    let r = reveal(&mut dp, &mut pool, &mut aml, &mut ev, d1, &mut || true);
    assert!(matches!(r, Revealed::Answered));
    assert!(matches!(
        reveal(&mut dp, &mut pool, &mut aml, &mut ev, d2, &mut || true),
        Revealed::Answered
    ));
    assert_eq!((dp.state(d1), pool.holders(pra), pool.holders(prb)), (Some(DState::D0), 2, 1));
    let down = settle(&mut dp, &mut pool, &mut aml, &mut ev, &[d1, d2], &mut |d| d == d2);
    assert_eq!(down, [d1], "DEV2 was claimed and stays on");
    assert_eq!((dp.state(d1), pool.holders(pra), pool.holders(prb)), (Some(DState::D3Cold), 1, 0));
}

#[test]
fn never_answers_and_goes_straight_back() {
    let (mut aml, _) = boot(&src(), 2);
    let [d1, _, _, _, pra, prb] = ids(&aml);
    let (mut probe, mut dp, mut pool) =
        (Probe::default(), DevicePower::default(), PowerPool::default());
    let r = reveal(&mut dp, &mut pool, &mut aml, &mut evaluator(&mut probe), d1, &mut || false);
    assert!(matches!(r, Revealed::Silent));
    assert_eq!(said(&probe), ints(&[0xB1, 0xA1, 0x40, 0x43, 0xA0, 0xB0]));
    assert_eq!((dp.state(d1), pool.holders(pra), pool.holders(prb)), (Some(DState::D3Cold), 0, 0));
    let down =
        settle(&mut dp, &mut pool, &mut aml, &mut evaluator(&mut probe), &[d1], &mut |_| false);
    assert!(down.is_empty(), "already down");
}
