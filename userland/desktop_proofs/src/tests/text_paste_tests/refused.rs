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

//! Bytes that are not text are refused.

use super::line::line;
use nonos_app_skeleton::input::text::paste_char;

#[test]
fn bytes_that_are_not_text_are_refused() {
    assert_eq!(line(b"ab\xff\xfecd"), None);
    assert_eq!(line(b"\xc3("), None);
    // Junk past the first line is never pasted; the line is, and the junk
    // counts as more.
    assert_eq!(line(b"name\n\xff\xfe"), Some(("name", true)));
}

#[test]
fn a_character_cut_by_the_buffer_is_dropped_and_the_rest_kept() {
    // A clipboard longer than the read buffer arrives cut, possibly inside
    // a character; what came before it is good text.
    assert_eq!(line(b"na\xc3\xafve \xe2\x82"), Some(("naïve", false)));
}

#[test]
fn a_tab_is_a_space_and_other_controls_are_nothing() {
    assert_eq!(paste_char('\t'), Some(' '));
    assert_eq!(paste_char('é'), Some('é'));
    for c in ['\u{0}', '\u{7}', '\u{1b}', '\u{7f}', '\u{85}'] {
        assert_eq!(paste_char(c), None, "{c:?}");
    }
}
