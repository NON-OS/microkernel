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

//! An image's head, held to the format before any member is read: the
//! header, the member table and the names. Each member's place follows from
//! the lengths before it, so the members tile the image from the end of the
//! names to its last byte and one image has one reading.

use alloc::vec::Vec;

use crate::format::{HEADER_LEN, MEMBER_LEN};
use crate::header::{header, Header};
use crate::member::{row, Member, Role};
use crate::refused::Refused;

#[derive(Debug)]
pub struct Head<'a> {
    pub total: u64,
    pub members: Vec<Member<'a>>,
}

impl<'a> Head<'a> {
    /// `b` is the image's first `head_len` bytes, or more of it.
    pub fn parse(b: &'a [u8]) -> Result<Self, Refused> {
        let h: Header = header(b)?;
        if b.len() < h.data_at() {
            return Err(Refused::Layout);
        }
        let (mut members, mut name_at, mut at) = (Vec::new(), h.names_at(), h.data_at() as u64);
        for i in 0..h.count {
            let r = row(&b[HEADER_LEN + i * MEMBER_LEN..HEADER_LEN + (i + 1) * MEMBER_LEN])?;
            let name_end = name_at + r.name_len;
            if name_end > h.data_at() {
                return Err(Refused::Layout);
            }
            let name = &b[name_at..name_end];
            if !crate::name::valid(name) || members.iter().any(|m: &Member| m.name == name) {
                return Err(Refused::Name);
            }
            // The listed package first, and only there.
            if (i == 0) != (r.role == Role::Listed) {
                return Err(Refused::Member);
            }
            let end = at.checked_add(r.stored).ok_or(Refused::Layout)?;
            if end > h.total {
                return Err(Refused::Layout);
            }
            let m = Member { name, role: r.role, codec: r.codec, at: at..end, raw: r.raw, digest: r.digest };
            members.push(m);
            (name_at, at) = (name_end, end);
        }
        if name_at != h.data_at() || at != h.total {
            return Err(Refused::Layout);
        }
        Ok(Head { total: h.total, members })
    }

    /// The member the listing sells.
    pub fn listed(&self) -> &Member<'a> {
        &self.members[0]
    }
}
