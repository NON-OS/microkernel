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

fn nth(i: usize) -> [u8; 32] {
    let mut m = [0u8; 32];
    m[..8].copy_from_slice(&(i as u64).to_le_bytes());
    m
}

#[test]
fn a_measurement_seen_again_is_not_absorbed_again() {
    let mut f = family(SUP);
    [1, 2, 1, 1, 2].iter().for_each(|&i| f.measured(&nth(i)));
    assert_eq!((f.list.as_slice(), f.n_meas), (&[nth(1), nth(2)][..], 2));
    assert_eq!(f.meas, meas_chain(&f.list));
}

#[test]
fn past_its_bound_m_stops_and_the_list_still_rebuilds_it() {
    let mut f = family(SUP);
    (0..MAX_MEAS + 3).for_each(|i| f.measured(&nth(i)));
    assert!(f.overflow);
    assert_eq!((f.list.len(), f.n_meas as usize), (MAX_MEAS, MAX_MEAS));
    assert_eq!(meas_chain(&f.list), f.meas, "the kept list is exactly M's");
}
