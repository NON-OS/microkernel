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

//! Peripherals that would take a service to report mode it should not
//! reach: one whose map has no pointer for its boot mouse, a Reference
//! longer than two octets,
//! descriptors past a Report's end, and a map cut off after a part that
//! parses alone.

use super::fallback_tests::boot;
use super::maps::{global, local, main, KEYBOARD};
use crate::sim::devices::{NOTIFY, READ};
use crate::sim::report_devices::*;
use crate::sim::{keyboard_mouse, Peripheral, Run};
use nonos_bt_hogp::host::{Kind, Outcome};
use nonos_bt_wire::att::error::UNLIKELY_ERROR;
use nonos_bt_wire::att::opcode::READ_BLOB_REQ;

#[test]
fn a_boot_mouse_with_no_pointer_in_the_map_keeps_boot_mode() {
    let db = report_keyboard(&KEYBOARD).char(READ | NOTIFY, 0x2a33, &[0; 3]).desc(0x2902, &[0, 0]);
    let mut run = Run::new(db);
    assert_eq!(run.go(), Some(Outcome::Ready { keyboard: true, mouse: true }));
    assert!(run.sent.iter().any(|p| p[..] == [0x0a, MAP as u8, 0]), "its map is read");
    assert_eq!(run.peer.writes[0], (MODE, vec![0], true), "but it types and cannot point");
    let kinds: Vec<_> = run.host.sources().iter().map(|s| s.kind).collect();
    assert_eq!(kinds, [Kind::Keyboard, Kind::Mouse]);
}

#[test]
fn a_reference_longer_than_two_octets() {
    boot(&mut Run::new(report_keyboard_with(&KEYBOARD, [&[1, 1, 0], &[2, 1], &[1, 2]])));
}

#[test]
fn descriptors_past_a_reports_end_are_not_its_own() {
    let mut peer = Peripheral::new(report_keyboard(&KEYBOARD));
    peer.wide = true;
    boot(&mut Run::with(peer));
}

#[test]
fn a_map_cut_off_after_a_part_that_parses_alone() {
    let keys = [global(0, 7), local(1, 0), local(2, 0x65), global(1, 0), global(2, 0x65)];
    let mut map = [keys.concat(), global(7, 8), global(9, 6), main(8, 0)].concat();
    map.resize(22, 0x34);
    map.extend([0x34; 10]);
    let mut p = Peripheral::new(report_keyboard_with(&map, [&[0, 1], &[9, 1], &[0, 2]]));
    p.refuse_op = Some((READ_BLOB_REQ, UNLIKELY_ERROR));
    boot(&mut Run::with(p));
}

#[test]
fn a_service_with_no_reports_never_has_its_map_read() {
    let db = keyboard_mouse();
    let map = db.handle_of(0x2a4b);
    let mut run = Run::new(db);
    assert_eq!(run.go(), Some(Outcome::Ready { keyboard: true, mouse: true }));
    assert!(!run.sent.iter().any(|p| p[..] == [0x0a, map as u8, 0]), "its map is not read");
    assert_eq!(run.peer.writes[0].1, [0], "boot mode");
}
