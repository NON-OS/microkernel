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

//! Which DMA grant a surface may be registered over (L35). Pure, so
//! kernel_proofs checks it by path.

/// The run (`physical_start`, `length`) of the grant whose user window
/// holds [`va`, `va + len`) whole, from `grants` as (`user_va`, `length`,
/// `physical_start`). None for an empty range, a range that wraps, or one
/// that reaches past a single grant, even into the next one: a surface's
/// frames must all come back to the broker as one run.
pub(crate) fn covering(
    grants: impl IntoIterator<Item = (u64, u64, u64)>,
    va: u64,
    len: u64,
) -> Option<(u64, u64)> {
    if len == 0 {
        return None;
    }
    let end = va.checked_add(len)?;
    grants.into_iter().find_map(|(base, length, physical)| {
        let top = base.checked_add(length)?;
        (base <= va && end <= top).then_some((physical, length))
    })
}
