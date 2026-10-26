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

//! Fresh entropy for a proof, drawn on the serving thread before the worker
//! starts, so the worker makes no call but the proof's own work.

/// 64 bytes from the kernel's pool, or None when it refuses.
pub fn drawn() -> Option<[u8; 64]> {
    let mut e = [0u8; 64];
    (nonos_libc::crypto_random(e.as_mut_ptr(), e.len()) >= 0).then_some(e)
}
