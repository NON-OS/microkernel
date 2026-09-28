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
//! Body step, continued.

use crate::body_step::{step, Step};

#[test]
fn the_deadline_ends_an_abandoned_transfer() {
    assert_eq!(step(0, false, true), Step::Stop);
}
#[test]
fn closing_outranks_the_deadline() {
    assert_eq!(step(0, true, true), Step::Drain);
}
#[test]
fn the_decision_table_is_complete_and_ends_only_where_it_should() {
    let mut ending = 0;
    for read in [0usize, 1] {
        for closed in [false, true] {
            for expired in [false, true] {
                let decision = step(read, closed, expired);
                if read > 0 {
                    assert_eq!(decision, Step::Keep);
                    continue;
                }
                if matches!(decision, Step::Drain | Step::Stop) {
                    ending += 1;
                }
            }
        }
    }
    assert_eq!(ending, 3, "closed twice and expired once, out of four empty-read rows");
}
