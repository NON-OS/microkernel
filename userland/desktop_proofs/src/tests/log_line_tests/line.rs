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

//! A line's tag and newline, and finding it in the log.

use nonos_app_skeleton::log_line::Line;

/* The Terminal's `log` filter (capsule_terminal builtin/log.rs). */
fn found_by(log: &[u8], word: &[u8]) -> Vec<Vec<u8>> {
    log.split(|&b| b == b'\n')
        .filter(|l| !l.is_empty())
        .filter(|l| l.windows(word.len()).any(|w| w.eq_ignore_ascii_case(word)))
        .map(<[u8]>::to_vec)
        .collect()
}

#[test]
fn a_line_is_tagged_and_ends_in_its_newline() {
    let line = Line::new(b"LAUNCH")
        .text(b"Calculator did not open: busy, try again (spawn answer ")
        .num(-16)
        .text(b")");
    assert_eq!(
        line.bytes(),
        b"[LAUNCH] Calculator did not open: busy, try again (spawn answer -16)\n"
    );
}

#[test]
fn log_area_finds_the_line_and_only_it() {
    let mut ring = Vec::new();
    ring.extend_from_slice(b"[SPAWN] name=app.calculator.1 pid=0x2A\n");
    ring.extend_from_slice(
        Line::new(b"LAUNCH").text(b"Calculator did not open: no window in 30 s").bytes(),
    );
    ring.extend_from_slice(
        Line::new(b"APP-FAIL")
            .text(b"Calculator: no window: wm open failed; the app ends (exit ")
            .num(3)
            .text(b")")
            .bytes(),
    );
    assert_eq!(
        found_by(&ring, b"launch"),
        [b"[LAUNCH] Calculator did not open: no window in 30 s".to_vec()]
    );
    assert_eq!(
        found_by(&ring, b"app-fail"),
        [b"[APP-FAIL] Calculator: no window: wm open failed; the app ends (exit 3)".to_vec()]
    );
}
