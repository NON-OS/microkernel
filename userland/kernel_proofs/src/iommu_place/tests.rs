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

//! a map whose write or flush fails is taken back before a
//! frame may be freed. The fake below keeps the tables and, apart, what the
//! unit sees: its view changes only on a flush that succeeds. Failed means
//! the unit sees none of the map; anything it might still see is Stranded.

use super::fake::run;
use super::place::Unplaced;

#[test]
fn an_honest_map_is_seen_whole() {
    assert_eq!(run(4, (None, None), Some(0)), (Ok(()), vec![true; 4]));
}

#[test]
fn a_failed_write_or_flush_is_taken_back_before_a_frame_is_freed() {
    assert_eq!(run(4, (Some(2), None), Some(0)), (Err(Unplaced::Failed(1)), vec![false; 4]));
    assert_eq!(run(4, (Some(0), None), Some(0)), (Err(Unplaced::Failed(1)), vec![false; 4]));
    // The first flush fails, the one after the clears succeeds.
    assert_eq!(run(4, (None, None), Some(1)), (Err(Unplaced::Failed(2)), vec![false; 4]));
}

#[test]
fn a_map_the_unit_may_still_see_is_stranded_and_frees_nothing() {
    assert_eq!(run(4, (None, None), None).0, Err(Unplaced::Stranded), "no flush succeeds");
    assert_eq!(
        run(4, (Some(3), Some(1)), Some(0)).0,
        Err(Unplaced::Stranded),
        "a leaf would not clear"
    );
}
