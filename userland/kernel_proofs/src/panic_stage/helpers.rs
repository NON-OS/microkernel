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

//! The claim each CPU makes, as the kernel's `end` does it: a
//! compare-and-swap from IDLE, then the stage rule on what it found.

use super::stage::{next, word, Next, IDLE, STOPPING};

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
