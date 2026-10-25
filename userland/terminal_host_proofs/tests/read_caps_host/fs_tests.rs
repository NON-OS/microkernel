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

//! slurp, and cat through it, at 256 KiB: a file of it read whole, one byte more refused.

use super::cat::cat;
use super::read_file::slurp;
use crate::clients::vfs;
use crate::fixtures::{lines, said, sized, state};

const MAX: usize = 256 * 1024;
const REFUSED: &[u8] = b"file larger than 256 KiB";

#[test]
fn slurp_reads_a_file_of_256_kib_whole() {
    vfs::put(b"/home/nonos/f", sized(MAX));
    let mut st = state();
    assert_eq!(slurp(&mut st, b"f"), Some(sized(MAX)));
    assert!(st.scrollback.rows.is_empty());
}

#[test]
fn slurp_refuses_a_file_one_byte_past_256_kib() {
    vfs::put(b"/home/nonos/f", sized(MAX + 1));
    let mut st = state();
    assert_eq!(slurp(&mut st, b"f"), None);
    assert_eq!(st.scrollback.rows, said(false, REFUSED));
}

#[test]
fn cat_prints_a_file_of_256_kib_whole() {
    vfs::put(b"/home/nonos/f", sized(MAX));
    let mut st = state();
    cat(&mut st, &[b"cat", b"f"]);
    assert_eq!(st.scrollback.rows, lines(&sized(MAX)));
}

#[test]
fn cat_refuses_a_file_one_byte_past_256_kib_and_prints_none_of_it() {
    vfs::put(b"/home/nonos/f", sized(MAX + 1));
    vfs::put(b"/home/nonos/g", b"next".to_vec());
    let mut st = state();
    cat(&mut st, &[b"cat", b"f", b"g"]);
    let mut want = said(false, REFUSED);
    want.push((false, b"next".to_vec()));
    assert_eq!(st.scrollback.rows, want);
}
