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

//! Moves between D-states: resources in ResourceOrder, shared ones held
//! until their last device lets go, and a fault mid-move undone.

use super::rig::{ids, ints, said, src};
use crate::device_power::{psc, DState, DevicePower, MoveError};
use crate::probe::Probe;
use crate::run::boot;
use crate::tests::platform::run::evaluator;

#[test]
fn resource_order_and_sharing() {
    let (mut aml, _) = boot(&src(), 2);
    let [d1, d2, _, _, pra, prb] = ids(&aml);
    let (mut probe, mut dp) = (Probe::default(), DevicePower::default());
    let mut ev = evaluator(&mut probe);
    dp.set(&mut aml, &mut ev, d1, DState::D3Cold, DState::D0).unwrap();
    dp.set(&mut aml, &mut ev, d2, DState::D3Cold, DState::D0).unwrap();
    assert_eq!((dp.pool.holders(pra), dp.pool.holders(prb)), (2, 1));
    dp.set(&mut aml, &mut ev, d1, DState::D0, DState::D3Cold).unwrap();
    dp.set(&mut aml, &mut ev, d2, DState::D0, DState::D3Cold).unwrap();
    drop(ev);
    // PRB (order 0) on before PRA though listed after it; off the other
    // way round; PRA stays on until DEV2 lets go.
    assert_eq!(said(&probe), ints(&[0xB1, 0xA1, 0x40, 0x43, 0xB0, 0xA0]));
    assert_eq!((dp.pool.holders(pra), dp.pool.holders(prb)), (0, 0));
    assert_eq!(dp.state(d1), Some(DState::D3Cold));
}

#[test]
fn a_fault_mid_move_is_undone() {
    let (mut aml, _) = boot(
        &[src(), crate::enc::store(&crate::enc::int(1), &crate::enc::name("\\FAIL"))].concat(),
        2,
    );
    let [d1, _, _, _, pra, prb] = ids(&aml);
    let (mut probe, mut dp) = (Probe::default(), DevicePower::default());
    let r = dp.set(&mut aml, &mut evaluator(&mut probe), d1, DState::D3Cold, DState::D0);
    assert!(matches!(r, Err(MoveError::Fault(_))));
    assert_eq!(said(&probe), ints(&[0xB1, 0xA1, 0xA0, 0xB0]), "both back off, highest order first");
    assert_eq!((dp.pool.holders(pra), dp.pool.holders(prb), dp.state(d1)), (0, 0, None));
    assert_eq!(psc(&mut aml, &mut evaluator(&mut probe), d1), Some(DState::D3Hot), "still in D3");
}
