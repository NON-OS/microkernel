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

//! The first publish pass's flag, over every order the steps can finish
//! in, with repeats: it goes up exactly once, only once all four are
//! done, so never before the records are published, and never again.

use crate::pass::{FirstPass, Step};

const STEPS: [Step; 4] = [Step::Namespace, Step::Osc, Step::Hidden, Step::Records];

/// Every sequence of `len` steps.
fn sequences(len: usize) -> Vec<Vec<Step>> {
    (0..4usize.pow(len as u32))
        .map(|mut n| {
            (0..len)
                .map(|_| {
                    let s = STEPS[n % 4];
                    n /= 4;
                    s
                })
                .collect()
        })
        .collect()
}

#[test]
fn up_exactly_once_and_only_after_every_step() {
    for seq in (1..=6).flat_map(sequences) {
        let mut p = FirstPass::default();
        let mut seen = Vec::new();
        let mut raised = 0;
        for s in seq {
            p.finish(s);
            seen.push(s);
            if p.raise() {
                raised += 1;
                assert!(STEPS.iter().all(|x| seen.contains(x)), "early: {seen:?}");
                assert!(seen.contains(&Step::Records), "before the records");
            }
            assert!(!p.raise(), "a second time");
        }
        let all = STEPS.iter().all(|x| seen.contains(x));
        assert_eq!(raised, usize::from(all), "{seen:?}");
    }
}
