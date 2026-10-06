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

//! An empty or blank /readme.txt parses to no blocks, which used to leave the
//! window blank after three reads of it; the page now names it instead.

use mdview_layout_tests::layout::parse;
use mdview_layout_tests::verdict::{empty_page, EMPTY};

#[test]
fn an_empty_file_is_named_not_drawn_blank() {
    assert_eq!(empty_page(parse("").len()), Some(EMPTY));
}

#[test]
fn a_file_of_blank_lines_is_named_not_drawn_blank() {
    assert_eq!(empty_page(parse("  \n\n\t\n").len()), Some(EMPTY));
}

#[test]
fn a_file_with_text_is_laid_out() {
    assert_eq!(empty_page(parse("# NONOS\n\nhello").len()), None);
}
