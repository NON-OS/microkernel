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

//! What the panel lets go of: a window seen shut, and everything the
//! service said once it cannot be reached.

use super::fixture::{answered, asked, two_bonds};
use crate::bluetooth::panel::{Panel, Reach};
use nonos_bt_proto::ops::{READY, SCANNING};
use nonos_bt_proto::{Asked, Link, NO_PLACE};

#[test]
fn a_window_seen_shut_is_not_counted_down() {
    let mut p = answered(SCANNING, Link::WINDOW);
    p.window_from_ms = Some(5);
    assert!(p.took_link(&Link { state: SCANNING, flags: Link::WINDOW, place: NO_PLACE }.write()));
    assert_eq!(p.window_from_ms, Some(5));
    assert!(p.took_link(&Link { state: SCANNING, flags: 0, place: NO_PLACE }.write()));
    assert_eq!(p.window_from_ms, None);
}

#[test]
fn a_service_lost_shows_nothing_it_said_before() {
    let mut p = answered(READY, Link::KEYBOARD);
    two_bonds(&mut p);
    asked(&mut p, Asked::Enter);
    assert!(p.digits.push(b'4'));
    (p.port, p.cursor, p.request_id, p.notice) = (Some(9), 1, 41, Some("kept"));
    p.lost(Reach::NoAnswer);
    assert_eq!((p.reach, p.port, p.link, p.health), (Reach::NoAnswer, None, None, None));
    assert_eq!((p.bond_count, p.level_count, p.asked), (0, 0, Asked::None));
    assert_eq!(p.digits.as_bytes(), b"");
    assert_eq!((p.cursor, p.request_id, p.notice), (1, 41, Some("kept")));
    let fresh = Panel::new();
    assert_eq!((fresh.reach, fresh.port), (Reach::Unasked, None));
}
