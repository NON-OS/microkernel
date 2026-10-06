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

//! Opening a file larger than the document can hold is refused, even when the
//! store cannot say the file's size first: the open reads one byte past the
//! capacity, so a file the read cut off never opens looking whole (and is
//! never saved back shortened).

use crate::open_limit::{refuse_open, READ_LIMIT};
use crate::state::CAPACITY;

/// What the vfs client hands back for a file of `file_len` bytes read with
/// `limit`: the bytes up to the limit, and nothing to say the file went on.
fn read(file_len: usize, limit: u32) -> Vec<u8> {
    vec![b'a'; file_len.min(limit as usize)]
}

#[test]
fn a_read_at_the_open_limit_tells_a_cut_off_file_from_one_that_fits() {
    let fits = read(CAPACITY, READ_LIMIT);
    let too_big = read(CAPACITY + 4096, READ_LIMIT);
    assert_ne!(fits.len(), too_big.len());
    // A read limited to the capacity itself could not: both come back alike.
    assert_eq!(read(CAPACITY, CAPACITY as u32), read(CAPACITY + 4096, CAPACITY as u32));
}

#[test]
fn a_file_larger_than_the_document_is_refused() {
    let why = refuse_open(&read(CAPACITY + 4096, READ_LIMIT)).expect("refused");
    assert_eq!(why, b"open refused: file is larger than 256 KiB");
}

#[test]
fn a_file_one_byte_over_is_refused() {
    assert!(refuse_open(&read(CAPACITY + 1, READ_LIMIT)).is_some());
}

#[test]
fn a_file_exactly_the_capacity_opens() {
    assert_eq!(refuse_open(&read(CAPACITY, READ_LIMIT)), None);
}

#[test]
fn an_empty_file_opens() {
    assert_eq!(refuse_open(b""), None);
}

#[test]
fn bytes_that_are_not_utf8_are_refused_for_that() {
    let why = refuse_open(&[b'a', 0xFF, b'b']).expect("refused");
    assert_eq!(why, b"open refused: file is not valid UTF-8");
}

#[test]
fn an_oversized_file_is_refused_for_its_size_before_its_encoding() {
    let mut bytes = read(CAPACITY + 1, READ_LIMIT);
    bytes[0] = 0xFF;
    assert_eq!(refuse_open(&bytes), Some(&b"open refused: file is larger than 256 KiB"[..]));
}
