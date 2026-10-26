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

//! What taking, giving, waiting and sweeping leave in a trip lock's rows.

use super::trip_rows::Rows;

const ME: u32 = 7;
const OTHER: u32 = 9;
const NOW: u64 = 1000;

#[test]
fn taking_giving_waiting_and_sweeping() {
    let mut r = Rows::new();
    r.wait(OTHER, false, NOW);
    r.wait(ME, true, NOW);
    r.wait(OTHER, true, NOW);
    assert_eq!((r.next, r.waiting.clone()), (Some((ME, NOW)), vec![OTHER, ME]));
    r.take(ME, true);
    assert_eq!((r.writer, r.next, r.waiting.clone()), (Some(ME), None, vec![OTHER]));
    assert!(r.has(ME) && !r.has(OTHER));
    assert_eq!(r.give(OTHER, true), (false, vec![]));
    assert_eq!(r.give(ME, true), (true, vec![OTHER]));
    r.take(ME, false);
    r.take(ME, false);
    r.take(OTHER, false);
    r.wait(5, true, NOW);
    assert_eq!(r.give(ME, false), (true, vec![]));
    assert!(!r.sweep(|p| p == ME || p == 5));
    assert_eq!((r.readers.clone(), r.next, r.waiting.clone()), (vec![(OTHER, 1)], None, vec![]));
    r.readers.clear();
    r.take(OTHER, true);
    assert!(r.sweep(|p| p == OTHER) && r.free());
}
