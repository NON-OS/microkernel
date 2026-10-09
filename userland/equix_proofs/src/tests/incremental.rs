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


//! Hashing in slices, the way the capsule runs it a little per tick, gives
//! exactly what one call gives, whatever the slice size.

use super::vectors::solves;
use nonos_equix::{HashX, Solution, Solver, INDEX_SPACE, MAX_SOLS};

fn sliced(challenge: &[u8], budget: u32) -> Vec<Solution> {
    let hashx = HashX::new(challenge).unwrap();
    let mut solver = Solver::new().unwrap();
    solver.start();
    let mut calls = 0u32;
    while !solver.hash_some(&hashx, budget) {
        calls += 1;
        assert!(calls <= INDEX_SPACE, "hashing never ends");
    }
    let mut out = [Solution::default(); MAX_SOLS];
    let n = solver.finish(&mut out);
    out[..n].to_vec()
}

#[test]
fn slice_size_does_not_matter() {
    let line = solves().into_iter().find(|l| l.solutions.len() >= 3).unwrap();
    for budget in [1, 7, 4096, 65535, INDEX_SPACE, u32::MAX] {
        assert_eq!(sliced(&line.challenge, budget), line.solutions, "budget {budget}");
    }
}

#[test]
fn finishing_before_hashing_is_done_finds_nothing() {
    let line = solves().into_iter().find(|l| !l.solutions.is_empty()).unwrap();
    let hashx = HashX::new(&line.challenge).unwrap();
    let mut solver = Solver::new().unwrap();
    solver.start();
    assert!(!solver.hash_some(&hashx, INDEX_SPACE - 1));
    let mut out = [Solution::default(); MAX_SOLS];
    assert_eq!(solver.finish(&mut out), 0);
    assert!(solver.hash_some(&hashx, 1));
    assert_eq!(solver.finish(&mut out), line.solutions.len());
}

#[test]
fn a_zero_budget_makes_no_progress_and_does_not_hang() {
    let line = solves().into_iter().next().unwrap();
    let hashx = HashX::new(&line.challenge).unwrap();
    let mut solver = Solver::new().unwrap();
    solver.start();
    for _ in 0..3 {
        assert!(!solver.hash_some(&hashx, 0));
    }
}

#[test]
fn starting_again_discards_a_half_finished_challenge() {
    let all = solves();
    let a = all.iter().find(|l| !l.solutions.is_empty()).unwrap();
    let b = all.iter().rev().find(|l| !l.solutions.is_empty()).unwrap();
    let mut solver = Solver::new().unwrap();
    solver.start();
    solver.hash_some(&HashX::new(&a.challenge).unwrap(), 30000);
    let hb = HashX::new(&b.challenge).unwrap();
    let mut out = [Solution::default(); MAX_SOLS];
    let n = solver.solve(&hb, &mut out);
    assert_eq!(out[..n].to_vec(), b.solutions);
}
