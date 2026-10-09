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

/* The store's side of `status_wire`: an answer read, or refused whole. */

use crate::status_wire::{Status, FETCHING, LEN, MAGIC, STARTING};

impl Status {
    /* An answer, when it is a whole one this layout knows. */
    pub fn decode(b: &[u8]) -> Option<Status> {
        if b.len() < LEN || b[..4] != MAGIC || !(FETCHING..=STARTING).contains(&b[4]) {
            return None;
        }
        let word = |i: usize| {
            u64::from_le_bytes(b[8 + i * 8..16 + i * 8].try_into().unwrap_or([0; 8]))
        };
        let s = Status {
            stage: b[4],
            route: b[5],
            total: word(0),
            done: word(1),
            rate: word(2),
            try_n: b[6],
            tries: b[7],
        };
        (s.done <= s.total).then_some(s)
    }
}
