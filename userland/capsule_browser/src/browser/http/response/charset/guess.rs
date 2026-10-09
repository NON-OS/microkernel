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

use super::{Encoding, WINDOWS_1252};

/* Bytes that are not UTF-8 decode as UTF-8 anyway while under one in
this many is invalid, so a stray byte in a UTF-8 page costs one
U+FFFD instead of the page. */
const INVALID_PER: usize = 100;

/* The encoding of bytes that declare none: UTF-8 when they are UTF-8,
or when they hold multi-byte UTF-8 sequences and under 1% of them is
invalid; otherwise windows-1252, the fallback of most locales. */
pub fn guess(b: &[u8]) -> Encoding {
    if core::str::from_utf8(b).is_ok() {
        return Encoding::Utf8;
    }
    let (mut multi, mut bad) = (0usize, 0usize);
    for chunk in b.utf8_chunks() {
        multi += chunk.valid().bytes().filter(|&c| c >= 0xC0).count();
        bad += chunk.invalid().len();
    }
    if multi > 0 && bad.saturating_mul(INVALID_PER) < b.len() {
        Encoding::Utf8
    } else {
        WINDOWS_1252
    }
}

/* A byte order mark: its encoding and length. */
pub fn bom(b: &[u8]) -> Option<(Encoding, usize)> {
    match b {
        [0xEF, 0xBB, 0xBF, ..] => Some((Encoding::Utf8, 3)),
        [0xFE, 0xFF, ..] => Some((Encoding::Utf16Be, 2)),
        [0xFF, 0xFE, ..] => Some((Encoding::Utf16Le, 2)),
        _ => None,
    }
}
