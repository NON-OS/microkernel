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

/// The glue that reads output calls libc; held to its order by its text.
#[test]
fn every_message_passes_the_hold_and_the_report_is_drawn_last() {
    let io = include_str!("../../../capsule_terminal/src/jobs/external_route.rs");
    assert!(io.contains("if boxed.is_some_and(|hold| hold.take(msg)) {\n        return;"));
    let drain = include_str!("../../../capsule_terminal/src/jobs/external_io.rs");
    assert_eq!(drain.matches("route(&buf[..m]").count(), 1);
    let ext = include_str!("../../../capsule_terminal/src/jobs/external.rs");
    let (ended, drawn) =
        (ext.find("out.program_ended()").unwrap(), ext.find("box_wait::ended(").unwrap());
    assert!(ended < drawn && ext.contains("route(&buf[..n]"));
    let draw = include_str!("../../../capsule_terminal/src/jobs/box_draw.rs");
    assert!(
        draw.find("out.feed_raw(SCREEN_RESET)").unwrap() < draw.find("out.writeln(l)").unwrap()
    );
    assert!(super::box_screen::SCREEN_RESET.starts_with(b"\x1b[?1049l"));
}

#[test]
fn the_key_under_any_other_tag_is_the_guests() {
    let mut hold = BoxHold::new(KEY);
    for tag in (0..=255u8).filter(|t| *t != TAG) {
        assert!(!hold.take(&[[tag].as_slice(), &KEY, b"x"].concat()), "tag {tag}");
    }
}

/*
 * The serial log's [BOX] line is only ever the one the keyed frames carried:
 * a guest that prints its own, plainly or in a frame dressed as a report's,
 * leaves nothing for the terminal to write there.
 */
#[test]
fn a_guest_printed_box_line_never_reaches_the_serial_log() {
    let mut hold = BoxHold::new(KEY);
    let line = b"[BOX] v=1 preset=strict refused=0 started=yes exit=0 source=capsule_linux\n";
    for f in [
        line.to_vec(),
        [[TAG].as_slice(), line].concat(),
        [&[TAG][..], &[0; KEY_LEN], line].concat(),
    ] {
        assert!(!hold.take(&f));
    }
    assert_eq!(hold.lines(), (Vec::new(), None));
    let draw = include_str!("../../../capsule_terminal/src/jobs/box_draw.rs");
    assert!(draw.contains("let (shown, line) = hold.lines();"));
    assert!(draw.contains("Some(l) => {\n            if !to_log(l) {"));
    /* The log's one writer, fed the keyed line or the terminal's own missing line. */
    assert_eq!(draw.matches("mk_debug(").count(), 1);
    assert!(draw.contains("bool {\n    super::box_log::log(line, |p| nonos_libc::mk_debug("));
    assert_eq!(draw.matches("to_log(").count(), 3);
    assert!(draw.contains("let l = missing_line(m, exit);\n    if !to_log(l.as_bytes()) {"));
}
