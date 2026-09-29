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

//! The bounds a header is held to. Each is far above what a real model needs
//! and far below what would let a header size an allocation by itself.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limits {
    pub max_tensors: u64,
    pub max_keys: u64,
    /// Bytes in one string: a key, a tensor name or a metadata value.
    pub max_string: u64,
    /// Items in one metadata array. A tokenizer's vocabulary is the largest.
    pub max_array: u64,
    /// One dimension of one tensor.
    pub max_dim: u64,
    /// Bytes of one tensor's data.
    pub max_tensor_bytes: u64,
}

/// Qwen2.5-0.5B has 291 tensors, 26 keys, a 151,936-entry vocabulary, and no
/// dimension above 151,936. A 7B model's largest tensor is under 1 GiB.
pub const DEFAULT: Limits = Limits {
    max_tensors: 16_384,
    max_keys: 16_384,
    max_string: 1 << 20,
    max_array: 1 << 24,
    max_dim: 1 << 31,
    max_tensor_bytes: 1 << 34,
};
