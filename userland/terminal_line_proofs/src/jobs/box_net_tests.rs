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

//! A box naming destinations sends its `[BOX-NET]` line in its keyed report,
//! before its `[BOX]` line: the terminal logs it and never draws it, and a
//! guest that prints one leaves nothing for the log.

use super::box_hold::{BoxHold, KEY_LEN, TAG};

const KEY: [u8; KEY_LEN] = [9; KEY_LEN];
const NET: &[u8] = b"[BOX-NET] v=1 allowed=10.0.2.2:80 connected=10.0.2.2:80";
const BOX: &[u8] = b"[BOX] v=1 preset=allow refused=0 started=yes exit=0";

fn keyed(text: &[u8]) -> Vec<u8> {
    [&[TAG][..], &KEY, text].concat()
}

#[test]
fn the_net_line_is_logged_and_never_drawn() {
    let mut hold = BoxHold::new(KEY);
    let report = [&b"BOX REPORT v1\n  connected  10.0.2.2:80\n"[..], NET, b"\n", BOX, b"\n"];
    assert!(hold.take(&keyed(&report.concat())));
    assert_eq!(hold.net_lines(), [NET]);
    let (shown, line) = hold.lines();
    assert_eq!(shown, [&b"BOX REPORT v1"[..], b"  connected  10.0.2.2:80"]);
    assert_eq!(line, Some(BOX));
    assert!(hold.complete());
}

#[test]
fn a_guest_printed_net_line_is_never_the_reports() {
    let mut hold = BoxHold::new(KEY);
    let line = [NET, b"\n"].concat();
    for f in
        [line.clone(), [&[TAG][..], &line].concat(), [&[TAG][..], &[0; KEY_LEN], &line].concat()]
    {
        assert!(!hold.take(&f));
    }
    assert!(hold.net_lines().is_empty() && !hold.complete());
}

/// The glue that writes the log, held to its order by its text.
pub fn the_log_writer_is_fed_the_keyed_lines_only() {
    let draw = include_str!("../../../capsule_terminal/src/jobs/box_draw.rs");
    assert!(draw.contains("let (shown, line) = hold.lines();"));
    let logged =
        "Some(l) => {\n            if !hold.net_lines().into_iter().chain([l]).all(to_log) {";
    assert!(draw.contains(logged));
    /* The log's one writer, fed the keyed lines or the terminal's own missing line. */
    assert_eq!(draw.matches("mk_debug(").count(), 1);
    assert!(draw.contains("bool {\n    super::box_log::log(line, |p| nonos_libc::mk_debug("));
    assert_eq!(draw.matches("to_log(").count(), 2);
    assert_eq!(draw.matches("to_log").count(), 3);
    assert!(draw.contains("let l = missing_line(m, exit);\n    if !to_log(l.as_bytes()) {"));
}
