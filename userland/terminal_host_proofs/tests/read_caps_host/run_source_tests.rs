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

//! A `<` file at 64 KiB: its lines fed whole, one byte more refused and nothing fed.

use super::run_source::read_input;
use crate::clients::vfs;
use crate::fixtures::{said, sized, state};

const MAX: usize = 64 * 1024;

#[test]
fn input_feeds_a_file_of_64_kib_whole() {
    vfs::put(b"/home/nonos/f", sized(MAX));
    let mut st = state();
    assert_eq!(read_input(&mut st, b"f"), vec![sized(MAX)]);
    assert!(st.scrollback.rows.is_empty());
}

#[test]
fn input_refuses_a_file_one_byte_past_64_kib_and_feeds_nothing() {
    vfs::put(b"/home/nonos/f", sized(MAX + 1));
    let mut st = state();
    assert!(read_input(&mut st, b"f").is_empty());
    assert_eq!(st.scrollback.rows, said(false, b"larger than a `<` file feeds (64 KiB)"));
}
