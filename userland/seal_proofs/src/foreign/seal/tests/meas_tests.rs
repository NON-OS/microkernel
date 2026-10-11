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

use nonos_proof_abi::{meas_chain, meas_step};

use super::fixture::{family, SUP};
use crate::foreign::seal::family::MAX_MEAS;
use crate::foreign::seal::meas::meas_step as kernel_step;

#[test]
fn the_kernels_m_is_the_verifiers() {
    let (m, x) = ([1u8; 32], [2u8; 32]);
    assert_eq!(kernel_step(&m, &x), meas_step(&m, &x));
    let mut f = family(SUP);
    let list = [[1u8; 32], [2; 32], [3; 32]];
    list.iter().for_each(|x| f.measured(x));
    assert_eq!((f.meas, f.n_meas, f.list.as_slice()), (meas_chain(&list), 3, &list[..]));
}

#[test]
fn a_list_past_its_bound_cannot_rebuild_m() {
    let mut f = family(SUP);
    for i in 0..=MAX_MEAS {
        f.measured(&[i as u8; 32]);
    }
    assert_eq!((f.list.len(), f.n_meas as usize), (MAX_MEAS, MAX_MEAS + 1));
    assert_ne!(meas_chain(&f.list), f.meas, "the kept list proves nothing");
}
