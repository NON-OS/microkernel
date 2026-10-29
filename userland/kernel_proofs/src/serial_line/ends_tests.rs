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

//! The endings no other proof reaches: lower-case hex, text whose own
//! newline ends the line (`format_args!` text), and the fatal writer's
//! path. Each finds its own line among every write the port took.

use super::line::Line;
use super::print::WRITES;

fn written(mark: &str) -> Vec<String> {
    let writes = WRITES.lock().unwrap();
    let text = writes.iter().map(|w| String::from_utf8_lossy(w).into_owned());
    text.filter(|s| s.starts_with(mark)).collect()
}

#[test]
fn hex_lower_is_the_same_number_in_lower_case() {
    let n = 0xDEAD_BEEF_0123_ABCD;
    Line::new().str(b"[LOWER] ").hex_lower(n).str(b" ").hex(n).end();
    assert_eq!(written("[LOWER]"), ["[LOWER] 0xdeadbeef0123abcd 0xDEADBEEF0123ABCD"]);
}

#[test]
fn text_ending_in_its_own_newline_is_one_line() {
    Line::new().str(b"[TEXT] said\r\n\n").end_text();
    assert_eq!(written("[TEXT]"), ["[TEXT] said"]);
    Line::new().str(b"[BARE] no newline").end_text();
    assert_eq!(written("[BARE]"), ["[BARE] no newline"]);
}

#[test]
fn a_fatal_line_goes_whole_through_the_fatal_writer() {
    Line::new().str(b"[FATAL-END] cpl=").dec(0).end_fatal();
    assert_eq!(written("[FATAL-END]"), ["[FATAL-END] cpl=0"]);
}
