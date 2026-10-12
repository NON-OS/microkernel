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

//! Moves refused before anything runs, and D3hot keeping _PR3.

use super::rig::{ids, src};
use crate::device_power::{DState, DevicePower, MoveError};
use crate::probe::Probe;
use crate::run::boot;
use crate::tests::platform::run::evaluator;

#[test]
fn refusals_and_d3hot() {
    let (mut aml, _) = boot(&src(), 2);
    let [_, d2, d3, d4, pra, prb] = ids(&aml);
    let (mut probe, mut dp) = (Probe::default(), DevicePower::default());
    let mut ev = evaluator(&mut probe);
    assert!(matches!(
        dp.set(&mut aml, &mut ev, d3, DState::D0, DState::D1),
        Err(MoveError::Unsupported)
    ));
    dp.set(&mut aml, &mut ev, d2, DState::D0, DState::D3Hot).unwrap();
    assert!(matches!(
        dp.set(&mut aml, &mut ev, d2, DState::D0, DState::D2),
        Err(MoveError::NotFromD0)
    ));
    dp.set(&mut aml, &mut ev, d2, DState::D0, DState::D0).unwrap();
    dp.set(&mut aml, &mut ev, d2, DState::D0, DState::D2).unwrap();
    assert_eq!(dp.pool.holders(pra), 1, "D2 holds _PR2");
    dp.set(&mut aml, &mut ev, d4, DState::D3Cold, DState::D0).unwrap();
    dp.set(&mut aml, &mut ev, d4, DState::D0, DState::D3Hot).unwrap();
    assert_eq!((dp.pool.holders(pra), dp.pool.holders(prb)), (1, 1), "D3hot keeps _PR3's PRB");
}
