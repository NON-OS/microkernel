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

//! Which recorded install gives way when the table is full.
//!
//! The table used to stop taking new listings once full, so the 33rd
//! install of a boot was never recorded and the store showed it as never
//! asked. Room is now made by dropping one finished job (installed,
//! removed, failed or refused), the one recorded longest ago. A job still
//! queued, installing or removing is never dropped: its answer is what the person is waiting for. Apart from the
//! table so the proofs can hold it (`kernel_proofs`).

/// Of `entries` (key, finished, when recorded), the key of the finished one
/// recorded first; None when every one is still moving.
pub(super) fn victim<'a, K: 'a>(
    entries: impl Iterator<Item = (&'a K, bool, u64)>,
) -> Option<&'a K> {
    entries.filter(|(_, finished, _)| *finished).min_by_key(|(_, _, at)| *at).map(|(k, _, _)| k)
}
