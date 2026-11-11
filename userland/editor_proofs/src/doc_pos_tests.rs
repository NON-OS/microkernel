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

//! The text buffer and the page model name the same place: every byte on a
//! char boundary maps to its block and offset, a heading's `#` prefix folded
//! away, and that pair maps back to the byte. The ribbon reads the caret's run
//! through the first map and a click on the page lands through the second.

use crate::edit_tests::doc;
use crate::mode::Mode;

#[test]
fn every_byte_maps_to_the_page_and_back() {
    let text = "# Heading\nSome b\u{f6}dy text.\n## Second\nThe end";
    let mut s = doc(text);
    s.mode = Mode::Document;
    s.reflow();
    for at in (0..=text.len()).filter(|&at| text.is_char_boundary(at)) {
        let (block, off) = s.doc_pos(at);
        let line_start = text[..at].rfind('\n').map_or(0, |n| n + 1);
        let prefix = text[line_start..].bytes().take_while(|&b| b == b'#').count();
        let folded = if prefix > 0 { prefix + 1 } else { 0 };
        if at - line_start >= folded {
            assert_eq!(s.doc_byte(block, off), at, "byte {at} does not come back");
        }
    }
}
