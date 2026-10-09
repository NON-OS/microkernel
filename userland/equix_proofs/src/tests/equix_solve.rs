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


//! The solver against the reference: for 32 challenges, the same solutions in
//! the same order. Order matters because a client submits the first solution
//! that passes the service's effort check.

use super::vectors::solves;
use nonos_equix::{HashX, Solution, Solver, MAX_SOLS};

fn solve(solver: &mut Solver, challenge: &[u8]) -> Vec<Solution> {
    let hashx = HashX::new(challenge).expect("the reference solved this challenge");
    let mut out = [Solution::default(); MAX_SOLS];
    let n = solver.solve(&hashx, &mut out);
    out[..n].to_vec()
}

#[test]
fn every_reference_solution_list_matches_in_order() {
    let all = solves();
    assert_eq!(all.len(), 32);
    let mut solver = Solver::new().expect("host heap");
    let mut total = 0;
    for line in &all {
        assert_eq!(solve(&mut solver, &line.challenge), line.solutions, "challenge {:02x?}", line.challenge);
        total += line.solutions.len();
    }
    // Equi-X averages about two solutions a challenge; the vectors include
    // challenges with none and challenges with five.
    assert!(all.iter().any(|l| l.solutions.is_empty()));
    assert!(all.iter().any(|l| l.solutions.len() >= 5));
    assert!(total > 40, "{total}");
}

#[test]
fn a_reused_solver_answers_as_a_fresh_one() {
    let all = solves();
    let mut reused = Solver::new().unwrap();
    for line in all.iter().rev().take(6) {
        let mut fresh = Solver::new().unwrap();
        assert_eq!(solve(&mut reused, &line.challenge), solve(&mut fresh, &line.challenge));
    }
}

#[test]
fn every_solution_found_is_in_canonical_order() {
    let mut solver = Solver::new().unwrap();
    for line in solves().iter().take(8) {
        for s in solve(&mut solver, &line.challenge) {
            assert!(s.in_order(), "{s:?}");
        }
    }
}
