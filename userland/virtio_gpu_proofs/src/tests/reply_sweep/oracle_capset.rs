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

//! GET_CAPSET_INFO's verdict, by the rules `oracle` states.

use super::device::Reply;
use super::oracle::{seen, word};

pub const CAPSET_ROOM: u32 = 24 + 16;

/// (capset id, max version, max size), or None when refused.
pub fn capset(r: &Reply) -> Option<(u32, u32, u32)> {
    let (b, claimed) = seen(r, CAPSET_ROOM);
    if claimed < CAPSET_ROOM || word(&b, 0) != 0x1102 {
        return None;
    }
    Some((word(&b, 24), word(&b, 28), word(&b, 32)))
}
