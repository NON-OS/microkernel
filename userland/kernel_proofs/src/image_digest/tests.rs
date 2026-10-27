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

use super::digest::{in_chunks, DigestError, CHUNK, MAX_ELF};
use std::cell::Cell;

/// The kernel's serve unit.
const UNIT: usize = 64 << 10;

/// A byte that follows its offset with no short period, so a chunk read from
/// the wrong place, twice or not at all changes the digest.
fn byte(at: u64) -> u8 {
    (at.wrapping_mul(0x9e37_79b9_7f4a_7c15) >> 56) as u8
}

fn digest(len: usize, unit: usize) -> Result<[u8; 32], DigestError> {
    let read = |at: u64, buf: &mut [u8]| -> Result<(), ()> {
        buf.iter_mut().zip(at..).for_each(|(b, i)| *b = byte(i));
        Ok(())
    };
    in_chunks(len as u64, unit, read, || {})
}

#[test]
fn the_chunked_digest_is_the_digest_of_the_whole() {
    let tail = 3 * CHUNK + 12_345;
    for len in [1, 2, UNIT - 1, UNIT + 1, CHUNK - 1, CHUNK, CHUNK + 1, 2 * CHUNK, tail] {
        let whole: Vec<u8> = (0..len as u64).map(byte).collect();
        let whole = *blake3::hash(&whole).as_bytes();
        assert_eq!(digest(len, UNIT), Ok(whole), "{len} bytes");
        assert_eq!(digest(len, 4093), Ok(whole), "{len} bytes in odd units");
    }
}

#[test]
fn an_empty_or_oversized_length_is_refused_before_any_read() {
    for len in [0, MAX_ELF as u64 + 1, (MAX_ELF + CHUNK) as u64, u64::MAX] {
        let reads = Cell::new(0);
        let read = |_: u64, _: &mut [u8]| -> Result<(), ()> {
            reads.set(reads.get() + 1);
            Err(())
        };
        assert_eq!(in_chunks(len, UNIT, read, || {}), Err(DigestError::Length), "{len}");
        assert_eq!(reads.get(), 0, "{len} bytes were read before the refusal");
    }
}

#[test]
fn an_image_of_max_elf_is_taken_whole() {
    let mut whole = blake3::Hasher::new();
    let pattern: Vec<u8> = (0..MAX_ELF as u64).map(byte).collect();
    for piece in pattern.chunks(999_983) {
        whole.update(piece);
    }
    assert_eq!(digest(MAX_ELF, UNIT), Ok(*whole.finalize().as_bytes()));
}
