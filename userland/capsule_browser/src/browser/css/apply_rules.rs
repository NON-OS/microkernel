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

use super::walk::Walker;

mod scan;

use scan::{collect, Scan};

impl<'a> Walker<'a> {
    /* The UA rules element `id` matches, in cascade order. The UA sheet
     * is never budgeted: base layout survives a hostile author sheet. */
    pub fn match_ua(&mut self, id: usize) {
        let Walker { dom, sib, ua, buckets, ua_hits, .. } = self;
        collect(Scan { dom, sib, sheet: *ua, index: ua.index, buckets }, id, ua_hits, None);
    }

    /* The author rules element `id` (or, with `pseudo`, one of its
     * pseudo-elements) matches, in cascade order. Each candidate test is
     * charged to the budget; once it is dry the element keeps its
     * inherited and UA style. */
    pub fn match_author(&mut self, id: usize, pseudo: bool) {
        let Walker { dom, sib, author, pseudo: pidx, buckets, author_hits, budget, .. } = self;
        let index = if pseudo { *pidx } else { author.index };
        let scan = Scan { dom, sib, sheet: *author, index, buckets };
        collect(scan, id, author_hits, Some(budget));
    }
}
