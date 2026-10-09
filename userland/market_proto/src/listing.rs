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

//! The catalogue, as `OP_LIST_APPS` returns it: a count, then per listing
//! its id, its 32-byte capsule measurement, its name, and one byte saying
//! whether any release of it is ready to install.

use alloc::vec::Vec;

use super::lp::{lp, skip, u32_at};

pub struct Entry {
    pub id: Vec<u8>,
    pub measurement: [u8; 32],
    pub name: Vec<u8>,
    pub ready: bool,
}

/// Every listing, or None when the body does not hold as many as its
/// count names.
pub fn parse_list(body: &[u8]) -> Option<Vec<Entry>> {
    let count = u32_at(body, 0)? as usize;
    let mut at = 4;
    // The count is the sender's word; the allocation is bounded by ours.
    let mut out = Vec::with_capacity(count.min(1024));
    for _ in 0..count {
        let (id, next) = lp(body, at)?;
        let measurement: [u8; 32] = body.get(next..skip(body, next, 32)?)?.try_into().ok()?;
        let (name, next) = lp(body, next + 32)?;
        let ready = *body.get(next)? != 0;
        at = next + 1;
        out.push(Entry { id: id.to_vec(), measurement, name: name.to_vec(), ready });
    }
    Some(out)
}
