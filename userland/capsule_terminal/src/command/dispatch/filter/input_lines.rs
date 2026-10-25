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

//! The lines a file gives a filter, and the arguments left once its files are taken out.

use alloc::vec::Vec;

/// A file's lines as a filter sees them: a newline ends a line rather than
/// starting an empty one, so `sort` of a file that ends in a newline does not
/// sort an empty line to the top.
pub fn lines_of(bytes: &[u8]) -> Vec<Vec<u8>> {
    if bytes.is_empty() {
        return Vec::new();
    }
    let body = bytes.strip_suffix(b"\n").unwrap_or(bytes);
    body.split(|&b| b == b'\n').map(<[u8]>::to_vec).collect()
}

/// `args` without the file operands, by identity, for running a filter over
/// files: what is left is the name and its flags.
pub fn without<'a>(args: &[&'a [u8]], files: &[&[u8]]) -> Vec<&'a [u8]> {
    args.iter()
        .copied()
        .filter(|a| {
            !files.iter().any(|f| core::ptr::eq(f.as_ptr(), a.as_ptr()) && f.len() == a.len())
        })
        .collect()
}
