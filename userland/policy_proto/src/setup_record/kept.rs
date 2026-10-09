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

/*
 * A string kept in a field of fixed size: a length byte, then the bytes,
 * then zeros to the end of the field. Copy, so a record stays plain data.
 */

use super::rules::{HOST_MAX, NAME_MAX, TIER_MAX};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Kept<const N: usize> {
    bytes: [u8; N],
    len: u8,
}

/* A kept name, held to `name_ok` wherever one is made. */
pub type Name = Kept<NAME_MAX>;
/* A kept Qwen tier, held to `tier_ok` wherever one is made. */
pub type Tier = Kept<TIER_MAX>;
/* A kept computer name, held to `host_ok` wherever one is made. */
pub type Host = Kept<HOST_MAX>;

impl<const N: usize> Kept<N> {
    pub const EMPTY: Self = Self { bytes: [0; N], len: 0 };

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len as usize]
    }

    pub(super) fn from_ok(s: &[u8], ok: fn(&[u8]) -> bool) -> Option<Self> {
        if s.len() > N || s.len() > u8::MAX as usize || !ok(s) {
            return None;
        }
        let mut bytes = [0u8; N];
        bytes[..s.len()].copy_from_slice(s);
        Some(Self { bytes, len: s.len() as u8 })
    }

    /* The field as written: length byte first, `N + 1` bytes in all. */
    pub(super) fn put(&self, out: &mut [u8]) {
        out[0] = self.len;
        out[1..1 + N].copy_from_slice(&self.bytes);
    }

    /* `None` unless the length fits, the padding is zeros and `ok` holds. */
    pub(super) fn take(field: &[u8], ok: fn(&[u8]) -> bool) -> Option<Self> {
        let (&len, rest) = field.split_first()?;
        let len = len as usize;
        if rest.len() != N || len > N || rest[len..].iter().any(|&b| b != 0) {
            return None;
        }
        Self::from_ok(&rest[..len], ok)
    }
}
