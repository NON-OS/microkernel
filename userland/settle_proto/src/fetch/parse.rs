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

//! Reading a FETCH request.

use super::escrow::Escrow;
use super::form::{Fetch, FETCH_ESCROW, FETCH_PAGE, FETCH_RELEASE, MAX_URL};

use crate::bytes::fixed;

impl Fetch {
    pub fn parse(b: &[u8]) -> Option<Self> {
        let (form, rest) = b.split_first()?;
        match *form {
            FETCH_RELEASE => {
                let len = usize::from(u16::from_le_bytes(fixed(rest, 0)?));
                let end = len.checked_add(2)?;
                if len > MAX_URL || Some(rest.len()) != end.checked_add(32) {
                    return None;
                }
                let url = rest.get(2..end)?.to_vec();
                Some(Fetch::Release { url, release_hash: fixed(rest, end)? })
            }
            FETCH_PAGE if rest.len() == 36 => Some(Fetch::Page {
                release_hash: fixed(rest, 0)?,
                offset: u32::from_le_bytes(fixed(rest, 32)?),
            }),
            FETCH_ESCROW => Escrow::parse(rest).map(Fetch::Escrow),
            _ => None,
        }
    }
}
