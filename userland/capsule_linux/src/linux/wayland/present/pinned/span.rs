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

//! Where a buffer's rows are for a pin: in its pool, in one mapping of its
//! memfd, from a page boundary in the guest.

use crate::linux::guest::memfd_spans::resolve;
use crate::linux::guest::MemfdSpan;

use super::super::super::scene::Scene;

pub const PAGE: u64 = 4096;

/// The guest address and byte length of `buffer`'s rows, all `height` of
/// them a whole `stride` long, when a pin can hold them.
pub fn span(scene: &Scene, spans: &[MemfdSpan], buffer: u32) -> Option<(u64, u64)> {
    let b = scene.buffers.iter().find(|b| b.id == buffer)?;
    let pool = scene.pools.iter().find(|p| p.id == b.pool)?;
    let len = u64::from(b.stride).checked_mul(u64::from(b.height))?;
    let end = b.offset.checked_add(len)?;
    if len == 0 || end > pool.size {
        return None;
    }
    let at = resolve(spans, pool.memfd, b.offset, len)?;
    (at % PAGE == 0).then_some((at, len))
}
