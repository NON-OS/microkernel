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

//! The markdown viewer never lays out a /readme.txt the read cut short and
//! never paints a blank page for one with nothing in it.

use crate::mdview_verdict::{empty_page, refuse_bytes, EMPTY, MAX_BYTES, READ_LIMIT, TOO_LARGE};

#[test]
fn a_read_at_the_limit_tells_a_cut_off_file_from_one_that_fits() {
    assert_eq!(READ_LIMIT as usize, MAX_BYTES as usize + 1);
    assert_eq!(refuse_bytes(MAX_BYTES as usize), None);
    assert_eq!(refuse_bytes(READ_LIMIT as usize), Some(TOO_LARGE));
    assert_eq!(refuse_bytes(0), None);
}

#[test]
fn a_page_with_no_blocks_is_named() {
    assert_eq!(empty_page(0), Some(EMPTY));
    assert_eq!(empty_page(1), None);
}

#[test]
fn both_refusals_name_the_file() {
    for why in [TOO_LARGE, EMPTY] {
        assert!(why.starts_with("mdview: /readme.txt is "));
    }
}
