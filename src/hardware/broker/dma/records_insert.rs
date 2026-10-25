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

//! A finished map entered in the grant table, under its claim (L69) and its
//! holder's quota (K10), both asked under the table's lock.

use super::super::super::quota;
use super::super::types::DmaGrant;
use super::RECORDS;

/// Entered only when `stands` holds of it, asked under the table's lock so a
/// drain that follows finds it: its claim, for the map path. None when the
/// claim is gone, Some(Err) when the quota is spent; either way the caller
/// undoes the map.
pub(in super::super) fn insert_if(
    record: DmaGrant,
    stands: impl FnOnce(&DmaGrant) -> bool,
) -> Option<Result<(), quota::Spent>> {
    let mut all = RECORDS.lock();
    if !stands(&record) {
        return None;
    }
    let mine = all.iter().filter(|g| g.pid == record.pid);
    let (held, bytes) = mine.fold((0, 0u64), |(n, b), g| (n + 1, b.saturating_add(g.length)));
    let (max, max_bytes) = (quota::MAX_DMA_GRANTS, quota::MAX_DMA_BYTES);
    if let Some(spent) = quota::spent(held, bytes, record.length, max, max_bytes) {
        return Some(Err(spent));
    }
    all.push(record);
    Some(Ok(()))
}
