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


//! Whether a recvmsg takes the descriptors owed: only into a control buffer
//! there is, with room for the whole block. Pure, so the host proofs hold it.

/// A block of `n` bytes into a buffer at `at` of `room` bytes.
pub fn handed(n: usize, at: u64, room: u64) -> bool {
    n > 0 && at != 0 && room >= n as u64
}
