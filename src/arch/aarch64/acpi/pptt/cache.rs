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

//! The Cache Type structure (type 1: 24 bytes, 28 from revision 3 with
//! the cache ID).

use super::super::le::{u16_at, u32_at, u8_at};

pub(crate) const CACHE_MIN: usize = 24;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cache {
    pub offset: u32,
    pub flags: u32,
    pub next_level: Option<u32>,
    pub size: u32,
    pub sets: u32,
    pub ways: u8,
    pub attributes: u8,
    pub line: u16,
    pub id: Option<u32>,
}

pub(crate) fn cache(s: &[u8], offset: u32, revision: u8) -> Option<Cache> {
    Some(Cache {
        offset,
        flags: u32_at(s, 4)?,
        next_level: u32_at(s, 8).filter(|n| *n != 0),
        size: u32_at(s, 12)?,
        sets: u32_at(s, 16)?,
        ways: u8_at(s, 20)?,
        attributes: u8_at(s, 21)?,
        line: u16_at(s, 22)?,
        id: if revision >= 3 && s.len() >= 28 { u32_at(s, 24) } else { None },
    })
}
