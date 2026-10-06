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

use super::matching::spent;

/* The rule index prunes candidates per node, so real pages stay far below
 * this; the cap only stops pathological sheets (huge universal buckets over
 * huge trees) from going quadratic. Budget exhaustion degrades styling from
 * the end of the document instead of dropping rules for the whole page. */
const MAX_SELECTOR_CHECKS: usize = 16_000_000;

/* Compound tests the matcher may spend in one cascade. A candidate test
 * can walk many ancestors and siblings, so counting candidates alone let a
 * 2 MB sheet of deep universal selectors hold the cascade for 8 s while
 * using under 1% of the candidate budget; this counts the real work. */
const MAX_MATCH_STEPS: usize = 50_000_000;

/* Total author-rule candidate tests allowed for one cascade. UA rules are
 * never budgeted: base block layout must survive a hostile author sheet. */
pub(super) struct MatchBudget {
    left: usize,
    /* The matcher's step count when this cascade began. */
    start: usize,
}

impl MatchBudget {
    pub(super) fn new() -> Self {
        MatchBudget { left: MAX_SELECTOR_CHECKS, start: spent() }
    }

    /* Reserve n candidate tests. False once dry, at which point the caller
     * skips author matching and keeps the inherited + UA style. */
    /* Dry also once the matcher has spent this cascade's steps. */
    pub(super) fn take(&mut self, n: usize) -> bool {
        if self.left < n || self.spent_out() {
            self.left = 0;
            return false;
        }
        self.left -= n;
        true
    }

    /* The matcher has spent this cascade's steps. Read before every
     * candidate too, since one candidate may spend a whole call's steps. */
    pub(super) fn spent_out(&self) -> bool {
        spent().wrapping_sub(self.start) > MAX_MATCH_STEPS
    }
}
