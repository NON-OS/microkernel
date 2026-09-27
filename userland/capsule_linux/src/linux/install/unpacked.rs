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

//! A database or package, decompressed by what its first bytes say it is.

use alloc::vec::Vec;

const GZIP: [u8; 2] = [0x1F, 0x8B];
const ZSTD: [u8; 4] = [0x28, 0xB5, 0x2F, 0xFD];
const XZ: [u8; 6] = [0xFD, 0x37, 0x7A, 0x58, 0x5A, 0x00];

/// A tar, decompressed if it is compressed.
pub fn unpacked(b: &[u8]) -> Option<Vec<u8>> {
    // An uncompressed tar says so at offset 257.
    let tar = b.get(257..262) == Some(b"ustar".as_slice());
    if tar {
        return Some(b.to_vec());
    }
    decompressed(b)
}

/// Bytes compressed with zstd, gzip or xz; anything else is None.
pub fn decompressed(b: &[u8]) -> Option<Vec<u8>> {
    if b.starts_with(&ZSTD) {
        return nonos_zstd::decompress(b);
    }
    if b.starts_with(&GZIP) {
        return nonos_inflate::gunzip(b);
    }
    b.starts_with(&XZ).then(|| nonos_xz::decompress(b))?
}
