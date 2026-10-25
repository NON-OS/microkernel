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

//! The leave schedule's legs side by side (rule.rs): waited on together,
//! not one after another.

use std::cell::Cell;

use super::rule::{run, Ended, Poll, TOTAL_MS};
use super::test_clock::clock;

#[test]
fn legs_are_waited_on_together_not_one_after_another() {
    // Each answers 4 s after it is first asked: four in turn would be 16 s,
    // past the total. Every leg is asked in the first round.
    let t = Cell::new(0);
    let first: [Cell<Option<u64>>; 4] = Default::default();
    let ended = run(
        &[5_000; 4],
        TOTAL_MS,
        clock(&t),
        |i| {
            let asked = *first[i].get().get_or_insert(t.get());
            first[i].set(Some(asked));
            if t.get() >= asked + 4_000 {
                Poll::Answered(0)
            } else {
                Poll::Waiting
            }
        },
        || {},
    );
    assert_eq!(ended, [Ended::Answered(0); 4]);
    assert!(first.iter().all(|f| f.get() == Some(2)), "every disk asked in the first round");
}
