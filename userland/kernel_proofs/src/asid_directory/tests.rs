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

use super::read::{complete, find};
use super::write::{hand_over, publish, withdraw};
use super::TURN;

/*
 * Asids far above any the race test uses, and withdrawn before the test
 * ends, so the tests share the one table without seeing each other.
 */
pub(super) const BASE: u32 = 0x4000_0000;

#[test]
fn an_address_space_is_found_by_asid_pid_and_root_until_withdrawn() {
    let _turn = TURN.lock().unwrap_or_else(|e| e.into_inner());
    publish(BASE + 1, 77, 0x1000);
    assert_eq!(find(|e| e.0 == BASE + 1), Some((BASE + 1, 77, 0x1000)));
    assert_eq!(find(|e| e.1 == 77).map(|e| e.0), Some(BASE + 1));
    assert_eq!(find(|e| e.2 == 0x1000).map(|e| e.0), Some(BASE + 1));
    withdraw(BASE + 1);
    assert_eq!(find(|e| e.0 == BASE + 1), None);
    assert!(complete(), "a miss proves absence while nothing overflowed");
}

#[test]
fn the_lowest_asid_answers_as_the_manager_table_did() {
    // Two spaces under one owner after a hand-over: the manager's table,
    // kept in asid order, answered the lower one; so does the directory,
    // whatever slots they landed in.
    let _turn = TURN.lock().unwrap_or_else(|e| e.into_inner());
    publish(BASE + 30, 5, 0x30_000);
    publish(BASE + 20, 6, 0x20_000);
    assert!(hand_over(BASE + 20, 5));
    assert_eq!(find(|e| e.1 == 5).map(|e| e.0), Some(BASE + 20));
    assert_eq!(find(|e| e.1 == 6), None, "the old owner still answers");
    assert_eq!(find(|e| e.0 == BASE + 20).map(|e| e.2), Some(0x20_000));
    assert!(!hand_over(BASE + 99, 5), "a hand-over of nothing succeeded");
    withdraw(BASE + 20);
    withdraw(BASE + 30);
}
