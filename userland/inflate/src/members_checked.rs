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

//! Every member of a gzip file, or why there are none: bad bytes, or an
//! allocator that could not give the output room.

use alloc::vec::Vec;

use super::gzip::MAX_MEMBERS;
use super::gzip_member::member;
use super::members::Member;
use super::types::End;

/// Why `members_checked` gave no members.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Refused {
    /// Not a whole, checked set of members within the bound.
    Bad,
    /// The allocator could not give this many bytes of output room.
    NoMemory(usize),
}

/// The same, saying whether the bytes were bad or memory ran out, so an
/// installer can tell the person which.
pub fn members_checked(data: &[u8], limit: usize) -> Result<Vec<Member>, Refused> {
    let mut out: Vec<Member> = Vec::new();
    let (mut at, mut total) = (0usize, 0usize);
    while at < data.len() {
        if out.len() == MAX_MEMBERS {
            return Err(Refused::Bad);
        }
        let m = member(data.get(at..).ok_or(Refused::Bad)?, limit - total);
        match m.end {
            End::Complete => {}
            End::NoMemory => return Err(Refused::NoMemory(m.short.unwrap_or(0))),
            _ => return Err(Refused::Bad),
        }
        total = total.checked_add(m.out.len()).ok_or(Refused::Bad)?;
        let end = at.checked_add(m.used).ok_or(Refused::Bad)?;
        if out.try_reserve(1).is_err() {
            return Err(Refused::NoMemory(core::mem::size_of::<Member>()));
        }
        out.push(Member { start: at, end, body: m.out });
        at = end;
    }
    if out.is_empty() {
        return Err(Refused::Bad);
    }
    Ok(out)
}
