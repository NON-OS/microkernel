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

//! BONDS's and BATTERY's replies: a count, then that many entries.

/// The most bonds the capsule keeps (`nonos_bt_bond::MAX_BONDS`, which the
/// capsule checks against this at build time).
pub const BONDS_MAX: usize = 15;
/// The most battery levels the capsule keeps: four Battery Services for
/// each of eight devices.
pub const LEVELS_MAX: usize = 32;
pub const BOND_LEN: usize = 9;
pub const LEVEL_LEN: usize = 3;
/// The longest reply, a full bond list, for a client's receive buffer.
pub const REPLY_MAX: usize = super::header::DATA_AT + 1 + BOND_LEN * BONDS_MAX;

/// The entries of a counted list, when the count, the bound and the length
/// agree.
pub(crate) fn entries(data: &[u8], len: usize, max: usize) -> Option<&[u8]> {
    let (&n, rest) = data.split_first()?;
    (usize::from(n) <= max && rest.len() == usize::from(n) * len).then_some(rest)
}
