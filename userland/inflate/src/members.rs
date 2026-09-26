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

//! A gzip file cut into its members, each with the bytes it occupies.

use alloc::vec::Vec;

use super::gzip::{verified, MAX_MEMBERS};
use super::tables::MAX_OUT;

/// One member: where its compressed bytes sit, and what they inflate to.
pub struct Member {
    pub start: usize,
    pub end: usize,
    pub body: Vec<u8>,
}

/// Every member of `data`, which must be nothing else. A signature covers a
/// byte range, so a byte that belongs to no member is not garbage to skip; it
/// is content that nothing vouched for, and the whole file is refused.
pub fn members(data: &[u8]) -> Option<Vec<Member>> {
    let mut out: Vec<Member> = Vec::new();
    let (mut at, mut total) = (0usize, 0usize);
    while at < data.len() {
        if out.len() == MAX_MEMBERS {
            return None;
        }
        let (body, len) = verified(data.get(at..)?)?;
        total = total.checked_add(body.len())?;
        if total > MAX_OUT {
            return None;
        }
        let end = at.checked_add(len)?;
        out.push(Member { start: at, end, body });
        at = end;
    }
    (!out.is_empty()).then_some(out)
}
