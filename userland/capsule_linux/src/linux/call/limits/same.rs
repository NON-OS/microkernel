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

//! Whether a `struct rlimit` a guest passed is a limit as it stands: a
//! soft limit then a hard one, each 64-bit little-endian. Pure, so the
//! host proofs hold it.

/// True when `asked` holds exactly `(soft, hard)`; a short struct is not.
pub fn same((soft, hard): (u64, u64), asked: &[u8]) -> bool {
    let word = |at: usize| asked.get(at..at + 8).and_then(|b| b.try_into().ok()).map(u64::from_le_bytes);
    asked.len() == 16 && word(0) == Some(soft) && word(8) == Some(hard)
}
