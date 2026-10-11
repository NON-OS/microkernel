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

//! A success outside the set, with the gate's row rewritten after the chains
//! were laid: each gate is consistent on its own, but no longer reads what
//! the chains committed to.

use isolation_stark::{forge, Statement};
use nonos_proof_abi::{hist_commit, syscall_root};
use nonos_stark::air::LiveGate as G;
use nonos_stark::field::Fp;

use super::fixture::witness;

// Gate b is trace 2 + b: two equal rows of G::WIDTH cells.
const PTRACE: usize = 2 + 101;

fn set(t: &mut [Fp], col: usize, v: u64) {
    (0..2).for_each(|r| t[r * G::WIDTH + col] = Fp::from_u64(v));
}

fn outside() -> (Statement, isolation_stark::Witness) {
    let mut w = witness();
    w.hist[2 * 101] = 1;
    let st =
        Statement { hc: hist_commit(&w.hist, &w.salt).unwrap(), a: syscall_root(&w.set).unwrap() };
    (st, w)
}

#[test]
fn a_gate_called_live_against_the_set_proves_nothing() {
    let (st, w) = outside();
    let tampered = forge(&st, &w, |t| {
        set(&mut t[PTRACE], G::LIVE, 1);
        set(&mut t[PTRACE], G::INV, 1);
    });
    assert!(tampered.is_err());
}

#[test]
fn a_success_dropped_from_the_gate_proves_nothing() {
    let (st, w) = outside();
    let tampered = forge(&st, &w, |t| {
        set(&mut t[PTRACE], G::value_col(0), 0);
        set(&mut t[PTRACE], G::INV, 0);
    });
    assert!(tampered.is_err());
}
