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

//! The header fields this reader needs.

use alloc::vec::Vec;

const NAME: usize = 100;
const LINK_AT: usize = 157;
const MAGIC_AT: usize = 257;
const PREFIX_AT: usize = 345;
const PREFIX: usize = 155;

/// The size field is octal text, space or NUL padded.
pub(super) fn octal(field: &[u8]) -> Option<usize> {
    let mut value = 0usize;
    for b in field {
        match b {
            b'0'..=b'7' => value = value.checked_mul(8)?.checked_add((b - b'0') as usize)?,
            b' ' | 0 => break,
            _ => return None,
        }
    }
    Some(value)
}

/// The path, with ustar's prefix in front when the header has one: ustar
/// splits a long path there rather than truncating it.
pub(super) fn name_of(head: &[u8]) -> Vec<u8> {
    let name = cstr(&head[..NAME]);
    if &head[MAGIC_AT..MAGIC_AT + 5] != b"ustar" {
        return name.to_vec();
    }
    let prefix = cstr(&head[PREFIX_AT..PREFIX_AT + PREFIX]);
    if prefix.is_empty() {
        return name.to_vec();
    }
    let mut out = prefix.to_vec();
    out.push(b'/');
    out.extend_from_slice(name);
    out
}

/// What a link entry points at.
pub(super) fn link_of(head: &[u8]) -> Vec<u8> {
    cstr(&head[LINK_AT..LINK_AT + NAME]).to_vec()
}

/// A field up to its first NUL.
pub(super) fn cstr(raw: &[u8]) -> &[u8] {
    &raw[..raw.iter().position(|b| *b == 0).unwrap_or(raw.len())]
}
