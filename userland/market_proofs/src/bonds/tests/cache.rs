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

//! The rows: a reading served until it is due, then read again, and the
//! oldest leaving a full cache.

use super::words::{reading, CHAIN, ID, MS, NOW};
use crate::bonds::{BondCache, Row, CACHE_MAX, REFRESH_AFTER};

#[test]
fn a_reading_is_served_and_read_again_once_due() {
    let mut c = BondCache::default();
    c.put(Row { key: (CHAIN, ID), reading: reading(NOW - 10), min: 7 });
    assert_eq!(c.reading((CHAIN, ID), NOW), Some((reading(NOW - 10), 7)));
    assert_eq!(c.next(MS), None, "fresh: nothing queued");
    let due = NOW - 10 + REFRESH_AFTER;
    assert!(c.reading((CHAIN, ID), due).is_some(), "still served");
    assert!(c.next(MS).is_some(), "and queued");
    assert_eq!(c.reading((1, ID), NOW), None, "another chain's bond is another row");
}

#[test]
fn a_full_cache_lets_the_oldest_reading_go() {
    let mut c = BondCache::default();
    let key = |i: usize| {
        let mut id = [1u8; 32];
        id[..2].copy_from_slice(&(i as u16).to_le_bytes());
        (CHAIN, id)
    };
    for i in 0..CACHE_MAX {
        c.put(Row { key: key(i), reading: reading(NOW - 1_000 + i as u64), min: 0 });
    }
    c.put(Row { key: key(5), reading: reading(NOW), min: 0 });
    assert!(c.reading(key(0), NOW).is_some(), "a replaced row evicts nothing");
    c.put(Row { key: key(CACHE_MAX), reading: reading(NOW), min: 0 });
    assert_eq!(c.reading(key(0), NOW), None, "the oldest went");
    assert!(c.reading(key(1), NOW).is_some());
    assert!(c.reading(key(CACHE_MAX), NOW).is_some());
}
