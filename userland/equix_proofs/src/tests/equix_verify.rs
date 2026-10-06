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


//! Verification: every reference solution passes, and each way of breaking
//! one is refused with its own reason. The permutation proof is the upstream
//! suite's: of the 40320 orderings of a solution's indices exactly one is
//! accepted, so a solution cannot be replayed reordered.

use super::vectors::{seeds, solves};
use nonos_equix::{verify, verify_with, HashX, Solution, VerifyError};

fn first_solution() -> (Vec<u8>, Solution) {
    let line = solves().into_iter().find(|l| !l.solutions.is_empty()).unwrap();
    (line.challenge, line.solutions[0])
}

#[test]
fn every_reference_solution_verifies() {
    let mut n = 0;
    for line in solves() {
        for s in &line.solutions {
            assert_eq!(verify(&line.challenge, s), Ok(()));
            n += 1;
        }
    }
    assert!(n > 40);
}

#[test]
fn a_swapped_pair_is_out_of_order() {
    let (challenge, mut s) = first_solution();
    s.idx.swap(0, 1);
    assert_eq!(verify(&challenge, &s), Err(VerifyError::Order));
}

#[test]
fn swapped_halves_are_out_of_order() {
    let (challenge, mut s) = first_solution();
    for i in 0..4 {
        s.idx.swap(i, i + 4);
    }
    assert_eq!(verify(&challenge, &s), Err(VerifyError::Order));
}

#[test]
fn indices_moved_across_pairs_break_a_partial_sum() {
    let (challenge, mut s) = first_solution();
    s.idx.swap(1, 2);
    assert_eq!(verify(&challenge, &s), Err(VerifyError::PartialSum));
}

#[test]
fn exactly_one_permutation_is_valid() {
    let (challenge, s) = first_solution();
    let hashx = HashX::new(&challenge).unwrap();
    let mut idx = s.idx;
    let mut valid = 0;
    permute(&mut idx, 0, &mut |p| {
        let candidate = Solution { idx: *p };
        if candidate.in_order() && verify_with(&hashx, &candidate).is_ok() {
            valid += 1;
        }
    });
    assert_eq!(valid, 1);
}

fn permute(idx: &mut [u16; 8], start: usize, f: &mut impl FnMut(&[u16; 8])) {
    if start == 7 {
        f(idx);
        return;
    }
    for i in start..8 {
        idx.swap(start, i);
        permute(idx, start + 1, f);
        idx.swap(start, i);
    }
}

#[test]
fn a_solution_does_not_carry_to_another_challenge() {
    let all = solves();
    let with: Vec<_> = all.iter().filter(|l| !l.solutions.is_empty()).collect();
    for pair in with.windows(2) {
        let s = &pair[0].solutions[0];
        assert!(verify(&pair[1].challenge, s).is_err());
    }
}

#[test]
fn a_challenge_without_a_program_refuses_everything() {
    let (bad, _) = seeds().into_iter().find(|(_, ok)| !ok).unwrap();
    let (_, s) = first_solution();
    assert_eq!(verify(&bad, &s), Err(VerifyError::Challenge));
}

#[test]
fn all_zero_indices_are_refused() {
    let (challenge, _) = first_solution();
    assert!(verify(&challenge, &Solution::default()).is_err());
}

#[test]
fn the_wire_form_round_trips_little_endian() {
    let (_, s) = first_solution();
    let bytes = s.to_bytes();
    assert_eq!(bytes[0], s.idx[0] as u8);
    assert_eq!(bytes[1], (s.idx[0] >> 8) as u8);
    assert_eq!(Solution::from_bytes(&bytes), s);
}
