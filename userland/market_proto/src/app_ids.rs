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

//! What a purchase needs of `OP_GET_APP` (B8): the listing's market id as
//! the market echoes it, its name as the market shows it, and the
//! publisher's key, which the listing's on-chain id is derived from
//!. The whole reply is read: a byte past the release count is
//! refused.

use alloc::vec::Vec;

use super::lp::{lp, skip, u32_at};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AppIds {
    /// The market id the reply is for, such as "linux.qwenchat".
    pub id: Vec<u8>,
    /// The listing's name, as the market shows it.
    pub name: Vec<u8>,
    pub publisher_key: [u8; 32],
}

pub fn parse_app_ids(body: &[u8]) -> Option<AppIds> {
    let (id, at) = lp(body, 0)?;
    let at = skip(body, at, 32)?;
    let (name, at) = lp(body, at)?;
    let (_, key_at) = lp(body, at)?;
    let at = skip(body, key_at, 32)?;
    let publisher_key = body.get(key_at..at)?.try_into().ok()?;
    let (_, at) = lp(body, at)?;
    u32_at(body, at)?;
    (at.checked_add(4)? == body.len()).then_some(())?;
    Some(AppIds { id: id.to_vec(), name: name.to_vec(), publisher_key })
}
