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

//! nox read and nox copy at 64 KiB: a file of it taken whole, one byte more refused.

use super::{copy, read};
use crate::clients::vfs;
use crate::fixtures::{lines, said, sized, state};

const MAX: usize = 64 * 1024;

#[test]
fn nox_read_shows_a_file_of_64_kib_whole() {
    vfs::put(b"/home/nonos/f", sized(MAX));
    let mut st = state();
    assert!(read::run(&mut st, &[b"f"]));
    assert_eq!(st.scrollback.rows, lines(&sized(MAX)));
}

#[test]
fn nox_read_refuses_a_file_one_byte_past_64_kib_and_shows_none_of_it() {
    vfs::put(b"/home/nonos/f", sized(MAX + 1));
    let mut st = state();
    assert!(!read::run(&mut st, &[b"f"]));
    assert_eq!(st.scrollback.rows, said(true, b"nox read: larger than it shows (64 KiB)"));
}

#[test]
fn nox_copy_writes_a_file_of_64_kib_whole() {
    vfs::put(b"/home/nonos/f", sized(MAX));
    let mut st = state();
    assert!(copy::run(&mut st, &[b"f", b"g"]));
    assert_eq!(vfs::read_file(9, b"/home/nonos/g", u32::MAX), Ok(sized(MAX)));
    assert_eq!(st.scrollback.rows, said(false, b"ok"));
}

#[test]
fn nox_copy_refuses_a_file_one_byte_past_64_kib_and_writes_nothing() {
    vfs::put(b"/home/nonos/f", sized(MAX + 1));
    let mut st = state();
    assert!(!copy::run(&mut st, &[b"f", b"g"]));
    assert!(vfs::writes().is_empty());
    assert!(vfs::read_file(9, b"/home/nonos/g", u32::MAX).is_err());
    assert_eq!(st.scrollback.rows, said(true, b"nox copy: larger than it copies (64 KiB)"));
}
