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

//! What one header is: an entry, a record about the next entry, or dropped.

use super::tar_field::{cstr, link_of, name_of};
use super::tar_path::member;
use super::tar_pax::{read as read_pax, Overrides};
use alloc::vec::Vec;

pub enum Kind {
    File,
    Symlink(Vec<u8>),
    Hardlink(Vec<u8>),
    Dir,
}

pub struct Entry {
    pub name: Vec<u8>,
    pub kind: Kind,
    pub body: Vec<u8>,
}

/// An entry; a pax, GNU long-name or global record; or a device, fifo or unknown.
pub(super) enum Read {
    Entry(Entry),
    Record,
    Dropped,
}

pub(super) fn read(flag: u8, head: &[u8], body: &[u8], next: &mut Overrides) -> Read {
    let kind = match flag {
        b'0' | 0 | b'7' => Kind::File,
        b'1' => Kind::Hardlink(member(next.link.take().unwrap_or_else(|| link_of(head)))),
        b'2' => Kind::Symlink(next.link.take().unwrap_or_else(|| link_of(head))),
        b'5' => Kind::Dir,
        b'x' => {
            read_pax(body, next);
            return Read::Record;
        }
        b'L' => {
            next.path = Some(cstr(body).to_vec());
            return Read::Record;
        }
        b'K' => {
            next.link = Some(cstr(body).to_vec());
            return Read::Record;
        }
        b'g' => return Read::Record, // global pax defaults: nothing reads them
        _ => {
            *next = Overrides::default();
            return Read::Dropped;
        }
    };
    let name = member(next.path.take().unwrap_or_else(|| name_of(head)));
    let body = if matches!(kind, Kind::File) { body.to_vec() } else { Vec::new() };
    *next = Overrides::default();
    Read::Entry(Entry { name, kind, body })
}
