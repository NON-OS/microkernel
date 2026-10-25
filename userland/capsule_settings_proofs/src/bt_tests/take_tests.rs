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

//! What the panel takes from each reply, and what it lets go of.

use super::fixture::{answered, asked, bond, list, two_bonds, KB, MOUSE};
use nonos_bt_proto::ops::{READY, SCANNING};
use nonos_bt_proto::{Asked, Link};

#[test]
fn a_shorter_bond_list_keeps_the_cursor_on_a_device() {
    let mut p = answered(READY, 0);
    two_bonds(&mut p);
    assert_eq!((p.bond_count, p.bonds[1].addr, p.bonds[1].linked), (2, MOUSE, true));
    p.cursor = 1;
    assert!(p.took_bonds(&list(&[bond(1, KB, true, false)])));
    assert_eq!((p.bond_count, p.cursor), (1, 0));
    assert!(p.took_bonds(&[0]));
    assert_eq!((p.bond_count, p.cursor), (0, 0));
}

#[test]
fn a_reply_the_panel_cannot_read_leaves_it_as_it_was() {
    let mut p = answered(SCANNING, Link::SCANNING);
    two_bonds(&mut p);
    let before = (p.link, p.bond_count, p.level_count);
    assert!(!p.took_link(&[9, 0, 0]));
    assert!(!p.took_bonds(&[3]));
    assert!(!p.took_levels(&[1, 0, 0, 200]));
    assert!(!p.took_asked(&[7, 0, 0, 0, 0]));
    assert!(!p.took_health(&[]));
    assert_eq!((p.link, p.bond_count, p.level_count), before);
}

#[test]
fn a_new_question_wipes_the_digits_typed_for_the_last() {
    let mut p = answered(SCANNING, Link::WAITING);
    asked(&mut p, Asked::Enter);
    "1234".bytes().for_each(|d| assert!(p.digits.push(d)));
    asked(&mut p, Asked::Enter);
    assert_eq!(p.digits.as_bytes(), b"1234", "the same question keeps them");
    asked(&mut p, Asked::None);
    assert_eq!(p.digits.as_bytes(), b"");
    assert_eq!(p.asked, Asked::None);
}
