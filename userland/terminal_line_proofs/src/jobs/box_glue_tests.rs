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

//! The terminal's half of the report's path that is glue around libc, and
//! a frame under any other tag.

use super::box_hold::{BoxHold, KEY_LEN, TAG};

const KEY: [u8; KEY_LEN] = [7; KEY_LEN];

/*
 * The glue that reads the program's output calls libc and is not mounted
 * here; held to the order it must keep by its text.
 */
#[test]
fn every_message_passes_the_hold_and_the_report_is_drawn_last() {
    let io = include_str!("../../../capsule_terminal/src/jobs/external_route.rs");
    assert!(io.contains("if boxed.is_some_and(|hold| hold.take(msg)) {\n        return;"));
    let drain = include_str!("../../../capsule_terminal/src/jobs/external_io.rs");
    assert_eq!(drain.matches("route(&buf[..m]").count(), 1);
    let ext = include_str!("../../../capsule_terminal/src/jobs/external.rs");
    let (ended, drawn) =
        (ext.find("out.program_ended()").unwrap(), ext.find("box_draw::draw(").unwrap());
    assert!(ended < drawn && ext.contains("route(&buf[..n]"));
    let draw = include_str!("../../../capsule_terminal/src/jobs/box_draw.rs");
    assert!(draw.find("out.feed_raw(b\"\\x18\")").unwrap() < draw.find("out.writeln(l)").unwrap());
}

#[test]
fn the_key_under_any_other_tag_is_the_guests() {
    let mut hold = BoxHold::new(KEY);
    for tag in (0..=255u8).filter(|t| *t != TAG) {
        assert!(!hold.take(&[[tag].as_slice(), &KEY, b"x"].concat()), "tag {tag}");
    }
}
