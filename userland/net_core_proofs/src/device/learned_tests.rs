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

//! The learned receive bound's rule: its floor, its ceiling, and what one
//! call teaches it.

use crate::device::learned_bound::{bound, learn, sample, BATCH, SINGLE};

#[test]
fn a_fresh_op_starts_at_its_floor_and_never_passes_its_ceiling() {
    assert_eq!(bound(&BATCH, 0), 24);
    assert_eq!(bound(&SINGLE, 0), 8);
    assert_eq!(bound(&BATCH, 14), 28);
    assert_eq!(bound(&BATCH, 100), 32);
    assert_eq!(bound(&SINGLE, 100), 24);
}

#[test]
fn a_timeout_teaches_at_least_the_wait() {
    assert_eq!(sample(false, 24, 24), 24);
    assert_eq!(sample(false, 30, 24), 30);
    assert_eq!(sample(true, 3, 24), 3);
    assert_eq!(learn(0, 8), 1);
    assert_eq!(learn(8, 8), 8);
}
