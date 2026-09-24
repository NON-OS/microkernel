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
//! gzip, including the concatenated members RFC 1952 allows.

use alloc::vec::Vec;

use super::gzip_header::body_at;
use super::inflate_raw::inflate_counted;
use super::tables::MAX_OUT;

/// A member ends with a CRC32 and an ISIZE.
const TRAILER: usize = 8;

/// Enough for a distribution index in several parts.
const MAX_MEMBERS: usize = 64;

/// Every member, concatenated.
pub fn gunzip(data: &[u8]) -> Option<Vec<u8>> {
    let mut out: Vec<u8> = Vec::new();
    let mut at = 0usize;
    for _ in 0..MAX_MEMBERS {
        let Some(start) = body_at(data.get(at..)?) else {
            return (at != 0).then_some(out);
        };
        let from = at.checked_add(start)?;
        let (mut part, used) = inflate_counted(data.get(from..)?)?;
        if out.len().checked_add(part.len())? > MAX_OUT {
            return None;
        }
        out.append(&mut part);
        at = from.checked_add(used)?.checked_add(TRAILER)?;
        if at >= data.len() {
            return Some(out);
        }
    }
    None
}
