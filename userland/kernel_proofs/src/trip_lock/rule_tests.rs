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

//! The trip lock's rows, whole tables over small row sets: who may come in,
//! and what taking, giving, waiting and sweeping leave behind.

use super::trip_rows::{Rows, FRESH_MS};

const ME: u32 = 7;
const OTHER: u32 = 9;
const NOW: u64 = 1000;

/// The rule, said plainly: no one in while a writer is; a writer only into
/// an empty lock no other fresh writer is next for; a reader unless a fresh
/// writer is next, or when it is already reading.
fn spec(r: &Rows, whole: bool) -> bool {
    let fresh_next = r.next.filter(|n| NOW - n.1 <= FRESH_MS).map(|n| n.0);
    let reading = r.readers.iter().any(|x| x.0 == ME);
    r.writer.is_none()
        && if whole {
            r.readers.is_empty() && fresh_next.is_none_or(|n| n == ME)
        } else {
            fresh_next.is_none() || reading
        }
}

#[test]
fn who_may_come_in_over_every_small_row_set() {
    let writers = [None, Some(ME), Some(OTHER)];
    let readers = [vec![], vec![(ME, 1)], vec![(OTHER, 2)], vec![(ME, 1), (OTHER, 1)]];
    let nexts = [None, Some((ME, NOW)), Some((OTHER, NOW)), Some((OTHER, NOW - FRESH_MS - 1))];
    let mut seen = 0;
    for writer in writers {
        for rd in &readers {
            for next in nexts {
                let r = Rows { writer, readers: rd.clone(), next, waiting: vec![] };
                for whole in [true, false] {
                    assert_eq!(r.may_take(ME, whole, NOW), spec(&r, whole), "{r:?} {whole}");
                    seen += 1;
                }
            }
        }
    }
    assert_eq!(seen, 96);
}

#[test]
fn a_mark_counts_through_fresh_ms_and_not_a_millisecond_more() {
    let r = Rows { next: Some((OTHER, NOW - FRESH_MS)), ..Rows::new() };
    assert_eq!(r.next_at(NOW), Some(OTHER));
    assert_eq!(r.next_at(NOW + 1), None);
}
