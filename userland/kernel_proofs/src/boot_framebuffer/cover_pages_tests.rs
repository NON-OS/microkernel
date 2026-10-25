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

//! The two steps the cover rule is built from (L79): a byte span widened to
//! the whole pages it touches, its start rounded down and its end up, and
//! two page spans meeting only when they share a page, never at an edge.

use super::cover_rule::{meets, pages};

#[test]
fn a_span_is_widened_to_the_whole_pages_it_touches() {
    let cases = [
        ((0x1234, 0x10), (0x1000, 0x2000)),
        ((0x1000, 0x1000), (0x1000, 0x2000)),
        ((0x1000, 0x1001), (0x1000, 0x3000)),
        ((0x1FFF, 2), (0x1000, 0x3000)),
        ((0x1800, 0x800), (0x1000, 0x2000)),
        ((0x2000, 0), (0x2000, 0x2000)),
    ];
    for ((base, len), want) in cases {
        assert_eq!(pages(base, len), want, "{base:#x}+{len:#x}");
    }
}

#[test]
fn page_spans_meet_on_a_shared_page_only() {
    let (a, b) = ((0x1000, 0x3000), (0x3000, 0x5000));
    assert!(!meets(a, b) && !meets(b, a), "an edge shared");
    let c = (0x2000, 0x4000);
    assert!(meets(a, c) && meets(c, a) && meets(b, c) && meets(c, b), "one page shared");
    let inner = (0x2000, 0x3000);
    assert!(meets(a, inner) && meets(inner, a), "one inside the other");
    let (far, wide) = ((0x8000, 0x9000), (0, 0x10_0000));
    assert!(!meets(a, far) && !meets(far, a), "pages apart");
    assert!(meets(far, wide) && meets(wide, far), "inside a wide span");
}
