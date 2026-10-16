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

//! A state reply's history: its newest entries that fit, and how many were left out.

use shield_wire::{Values, BODY_MAX};

use super::seen::Seen;

pub(super) fn history(v: &mut Values, s: &Seen) {
    /* The history goes last, its newest entries that fit the reply
     * whole; how many older ones were left out is said. */
    match &s.history {
        Ok(all) => {
            v.put("history", "1");
            let room = BODY_MAX.saturating_sub(v.text().len() + CUT_LINE);
            let left = v.put_newest("entry", all, room);
            if left > 0 {
                v.put("history_cut", &left.to_string());
            }
        }
        Err(why) => {
            v.put("history_why", why);
        }
    }
}

/* Room kept for the `history_cut=<count>` line after the entries. */
const CUT_LINE: usize = 32;
