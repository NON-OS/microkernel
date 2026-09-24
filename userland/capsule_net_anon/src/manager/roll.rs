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

//! A random value for one draw.

use nonos_libc::crypto_random;

pub(super) fn roll() -> Option<u64> {
    let mut bytes = [0u8; 8];
    if crypto_random(bytes.as_mut_ptr(), bytes.len()) != bytes.len() as i64 {
        return None;
    }
    Some(u64::from_le_bytes(bytes))
}
