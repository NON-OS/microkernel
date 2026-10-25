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

//! Claims the witness does not back. The forger's prover makes each proof
//! with the honest machinery; none of them may verify.

use isolation_stark::{forge, Statement};
use nonos_proof_abi::{hist_commit, syscall_root};

use super::fixture::{proven, witness};

#[test]
fn the_forge_path_itself_proves_an_honest_claim() {
    let (st, _) = proven();
    assert!(forge(st, &witness(), |_| {}).is_ok());
}

/// plan/12's case: ok_b > 0 with a_b = 0, under the true Hc and A.
#[test]
fn a_success_outside_the_set_proves_nothing() {
    let mut w = witness();
    w.hist[2 * 101] = 1;
    assert!(Statement::of(&w).is_none());
    let claim =
        Statement { hc: hist_commit(&w.hist, &w.salt).unwrap(), a: syscall_root(&w.set).unwrap() };
    assert!(forge(&claim, &w, |_| {}).is_err());
}

#[test]
fn another_commitment_or_set_proves_nothing() {
    let (st, _) = proven();
    let mut hc = *st;
    hc.hc[0] ^= 1;
    let mut a = *st;
    a.a[3] ^= 1;
    for claim in [hc, a] {
        assert!(forge(&claim, &witness(), |_| {}).is_err(), "{claim:?}");
    }
}
