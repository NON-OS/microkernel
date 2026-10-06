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

//! The bounds every read is held to.

/// The most one call may produce, across all frames.
pub const MAX_OUT: usize = 64 * 1024 * 1024;

/// Block_Maximum_Size: no block regenerates more than this.
pub const BLOCK_MAX: usize = 128 * 1024;

/// Largest accuracy each table may declare.
pub const LL_LOG: u8 = 9;
pub const ML_LOG: u8 = 9;
pub const OF_LOG: u8 = 8;
pub const WEIGHT_LOG: u8 = 6;

/// Largest symbol each alphabet has.
pub const LL_MAX: usize = 35;
pub const ML_MAX: usize = 52;
pub const OF_MAX: usize = 31;
pub const WEIGHT_MAX: usize = 12;

/// Longest Huffman code a literal may have.
pub const HUFF_LOG: u8 = 11;
