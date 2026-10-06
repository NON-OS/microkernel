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

//! The tray's labels reach the menu bar: the table hands over every live
//! label, and the bar shows the ones that fit with a "+N" for the rest.

use crate::protocol::TRAY_LABEL_MAX;
use crate::tray::fit::{fit, span};
use crate::tray::{TrayEntry, TrayTable};

fn item(owner_pid: u32, tray_id: u32, text: &[u8]) -> TrayEntry {
    let mut label = [0u8; TRAY_LABEL_MAX];
    label[..text.len()].copy_from_slice(text);
    TrayEntry { owner_pid, tray_id, label_len: text.len() as u32, label, in_use: true }
}

#[test]
fn every_live_label_is_handed_to_the_bar_and_a_removed_one_is_not() {
    let mut t = TrayTable::new();
    assert!(t.insert(item(20, 1, b"VPN on")).is_ok());
    assert!(t.insert(item(21, 1, b"Sync 3")).is_ok());
    assert!(t.remove(20, 1));
    let shown: Vec<&[u8]> = t.labels().collect();
    assert_eq!(shown, vec![&b"Sync 3"[..]]);
}

#[test]
fn labels_that_fit_are_all_shown() {
    assert_eq!(fit(&[40, 30], 100, 10, 20), 2);
    assert_eq!(span(&[40, 30], 10), 80);
}

#[test]
fn labels_that_do_not_fit_leave_room_for_the_count_of_the_rest() {
    // 40 + 10 + 30 + 10 + 30 = 120 > 100: two shown need 80, then a gap and
    // the 20 wide mark, 110, too wide; one shown needs 40 + 10 + 20 = 70.
    assert_eq!(fit(&[40, 30, 30], 100, 10, 20), 1);
}

#[test]
fn a_bar_with_room_only_for_the_mark_shows_the_mark_alone() {
    assert_eq!(fit(&[90, 90], 25, 10, 20), 0);
}

#[test]
fn no_labels_need_no_room() {
    assert_eq!(fit(&[], 0, 10, 20), 0);
    assert_eq!(span(&[], 10), 0);
}
