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

impl Fetch {
    pub fn parse(b: &[u8]) -> Option<Self> {
        let (form, rest) = b.split_first()?;
        match *form {
            FETCH_RELEASE => {
                let len = u16::from_le_bytes(rest.get(..2)?.try_into().ok()?) as usize;
                if len > MAX_URL || rest.len() != 2 + len + 32 {
                    return None;
                }
                let url = rest[2..2 + len].to_vec();
                Some(Fetch::Release { url, release_hash: rest[2 + len..].try_into().ok()? })
            }
            FETCH_PAGE if rest.len() == 36 => Some(Fetch::Page {
                release_hash: rest[..32].try_into().ok()?,
                offset: u32::from_le_bytes(rest[32..].try_into().ok()?),
            }),
            FETCH_ESCROW => Escrow::parse(rest).map(Fetch::Escrow),
            _ => None,
        }
    }
}
