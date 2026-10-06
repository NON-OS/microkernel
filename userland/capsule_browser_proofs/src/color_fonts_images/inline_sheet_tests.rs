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

//! The inline <style> text budget.

use super::sheet_tests::color;
use crate::probe::Page;

#[test]
fn a_600_kb_inline_sheet_applies_its_last_rule() {
    /* Rules of about 300 bytes, as a data-URI-heavy sheet has, so the
     * text budget and not the parser's rule count is what is tested. */
    let (mut css, pad) = (String::new(), "A".repeat(280));
    while css.len() < 600 * 1024 {
        css.push_str(&format!(".c{} {{ background: url(data:x;base64,{pad}) }}\n", css.len()));
    }
    css.push_str("#x { color: #ff0000 }");
    let html = format!("<style>{css}</style><p id=x>x</p>");
    assert_eq!(color(&Page::at(&html, (1336, 680)), "x"), 0xFFFF_0000);
}
