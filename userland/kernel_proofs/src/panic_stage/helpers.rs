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

//! The claim each CPU makes, as the kernel's `end` does it (a compare-and-
//! swap from IDLE, then the stage rule on what it found), and `terminate`'s
//! entry beside it.

use super::stage::{next, taken, word, Next, IDLE, STOPPING};

/// What `end` does for `me`, the word left behind, as the kernel runs it.
pub(super) fn enter(state: &mut u64, me: u64) -> Next {
    let found = if *state == IDLE {
        *state = word(STOPPING, me);
        IDLE
    } else {
        *state
    };
    next(found, me)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Did {
    Leave,
    Panic(Next),
    Parked,
}

/// `terminate`'s entry, as terminate.rs runs it.
pub(super) fn terminate(stage: u64, ending: &mut bool) -> Did {
    if taken(stage) || core::mem::replace(ending, true) {
        Did::Parked
    } else {
        Did::Leave
    }
}
