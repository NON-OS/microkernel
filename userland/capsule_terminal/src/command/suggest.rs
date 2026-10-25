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

//! "did you mean": the nearest command to something that was not one.
//!
//! A mistyped command is nearly always a typo or a name from another system,
//! and both have an obvious neighbour. Printing only that a name is unknown
//! makes the reader go and look it up; printing the neighbour usually ends the
//! problem on the spot.
//!
//! The distance is Levenshtein, bounded, over ASCII. Command names are short
//! and the candidate list is under a hundred, so the cost of this is invisible
//! next to spawning anything.

#[path = "suggest_distance.rs"]
mod suggest_distance;
#[path = "suggest_nearest.rs"]
mod suggest_nearest;

pub use suggest_distance::distance;
pub use suggest_nearest::nearest_two;

/// Longest name this will compare. Anything longer is not a mistyped command.
const MAX_LEN: usize = 32;

/// Edits allowed before two names are simply different words. Two covers a
/// transposition and a slip; three starts proposing `grep` for `help`.
const MAX_EDITS: usize = 2;
