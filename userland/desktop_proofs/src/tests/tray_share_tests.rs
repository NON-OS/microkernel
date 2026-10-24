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

//! The desktop shell's tray: one client holds at most half the items, and
//! the items of a client that ended are removed while a living client's stay.

use crate::tray::{TrayEntry, TrayTable};

const A: u32 = 20;
const B: u32 = 21;
const C: u32 = 22;

fn item(owner_pid: u32, tray_id: u32) -> TrayEntry {
    TrayEntry { owner_pid, tray_id, label_len: 1, in_use: true, ..TrayEntry::default() }
}

/// Register items for `owner` until the tray refuses it, and say how many.
fn fill(t: &mut TrayTable, owner: u32) -> u32 {
    let mut id = 1;
    while t.insert(item(owner, id)).is_ok() {
        id += 1;
    }
    id - 1
}

/// How many items the whole tray holds: register for one client after
/// another until a new client is refused too.
fn capacity() -> u32 {
    let mut t = TrayTable::new();
    (100..).map(|owner| fill(&mut t, owner)).take_while(|&n| n > 0).sum()
}

/*
 * Tray ids are the client's own, so one client could register every item
 * under ids it made up, and every other program's register was refused.
 */
#[test]
fn a_client_stops_at_half_the_tray_and_another_still_registers() {
    let mut t = TrayTable::new();
    let share = fill(&mut t, A);
    assert_eq!(share as usize, t.held_by(A));
    assert_eq!(2 * share, capacity(), "half the tray, and no more");
    assert!(t.insert(item(B, 1)).is_ok(), "another client still registers");
}

/*
 * Only the owner removes its item, so a tray app that crashed left its item
 * and its place for good.
 */
#[test]
fn an_ended_client_s_items_are_removed_and_its_places_reused() {
    let mut t = TrayTable::new();
    let a = fill(&mut t, A);
    let b = fill(&mut t, B);
    assert!(t.insert(item(C, 1)).is_err(), "the tray is full");
    assert_eq!(t.remove_ended(|pid| pid != A), a as usize);
    assert_eq!(t.held_by(A), 0);
    assert!(t.find(A, 1).is_none(), "A's item is gone");
    assert!((1..=b).all(|id| t.find(B, id).is_some()), "B's stay");
    assert!(t.insert(item(C, 1)).is_ok(), "a new client registers");
    assert_eq!(t.remove_ended(|_| true), 0, "nobody ended, nothing removed");
}
