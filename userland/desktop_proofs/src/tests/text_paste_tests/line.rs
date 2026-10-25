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

//! A field takes the first line, trimmed.

use nonos_app_skeleton::input::text::{first_line, PasteLine};

pub(super) fn line(bytes: &[u8]) -> Option<(&str, bool)> {
    first_line(bytes).map(|PasteLine { text, more }| (text, more))
}

#[test]
fn a_field_takes_the_first_line_trimmed() {
    assert_eq!(line(b"hello"), Some(("hello", false)));
    assert_eq!(line(b"  word \n"), Some(("word", false)), "the copied newline is not more");
    assert_eq!(line(b"one\r\ntwo"), Some(("one", true)));
    assert_eq!(line(b"\n\nlater"), Some(("later", false)), "leading blank lines skipped");
    assert_eq!(line(b"caf\xc3\xa9"), Some(("café", false)));
    assert_eq!(line(b""), Some(("", false)), "an empty clipboard is an empty line");
}
