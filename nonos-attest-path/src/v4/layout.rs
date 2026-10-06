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

/*
 * Layout, little-endian, nothing may follow:
 *
 *   0   8   MAGIC_V4
 *   8   1   kind (0 kernel, 1 capsule, 3 bootloader)
 *   9   4   v3 path length
 *   13  n   v3 path trailer, its own magic first
 *   ..  4   STARK proof length, at most the caller's bound
 *   ..  m   STARK proof bytes, whose own header names its parameter id
 */

pub const MAGIC_V4: [u8; 8] = *b"NATTV4\0\0";

/// The largest proof any parameter id may carry. A gate passes a bound no
/// larger than this, taken from its pinned parameter id.
pub const MAX_PROOF_V4: usize = 256 * 1024;

pub(super) const HEAD: usize = 13;
pub(super) const KIND_AT: usize = 8;
pub(super) const PATH_LEN_AT: usize = 9;
