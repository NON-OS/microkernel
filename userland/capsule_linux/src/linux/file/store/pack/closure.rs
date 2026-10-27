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

//! A program's closure: the name of every package its install needed, one
//! a line, as the install records it beside the program's record. A run of
//! the program serves only these packages' packs; a run with no closure,
//! of a program installed before closures were kept or of no package at
//! all, serves every pack of its family.

use alloc::string::String;
use alloc::vec::Vec;

/// The most names a closure holds: the most packages an install may bring
/// (install/limit.rs).
pub const MAX_NAMES: usize = 4096;
/// The most bytes a closure is.
pub const MAX_BYTES: u32 = 256 << 10;

/// The closure as it is kept, or None when it cannot be kept whole: past
/// either bound, or with a name that is empty or would break its line.
pub fn encode(names: &[String]) -> Option<Vec<u8>> {
    let bad = |n: &String| n.is_empty() || n.bytes().any(|b| b == b'\n');
    if names.len() > MAX_NAMES || names.iter().any(bad) {
        return None;
    }
    let out: Vec<u8> = names.iter().flat_map(|n| n.bytes().chain(Some(b'\n'))).collect();
    (out.len() as u64 <= u64::from(MAX_BYTES)).then_some(out)
}

/// The names a kept closure holds, or None when it is not one encode keeps:
/// past either bound, cut short of its last line, or not text.
pub fn decode(bytes: &[u8]) -> Option<Vec<String>> {
    if bytes.len() as u64 > u64::from(MAX_BYTES) || bytes.last().is_some_and(|b| *b != b'\n') {
        return None;
    }
    let lines = bytes.split(|b| *b == b'\n').filter(|l| !l.is_empty());
    let names: Option<Vec<String>> =
        lines.map(|l| core::str::from_utf8(l).ok().map(String::from)).collect();
    names.filter(|n| n.len() <= MAX_NAMES)
}

/// Whether a run with `closure` serves `package`'s pack.
pub fn serves(closure: Option<&[String]>, package: &[u8]) -> bool {
    closure.is_none_or(|names| names.iter().any(|n| n.as_bytes() == package))
}
