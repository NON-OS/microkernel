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

//! What a GGUF header may not do, each refusal with what was found.

use crate::tensor_error::TensorError;

/// Why a GGUF file was refused. Offsets are bytes from the file's start;
/// `tensor` is the tensor's index in the header.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GgufError {
    /// The reader could not supply bytes that the file's length says exist.
    Source {
        at: u64,
    },
    /// The file ends inside a field that starts at `at`.
    Truncated {
        at: u64,
    },
    BadMagic,
    UnsupportedVersion(u32),
    TooManyTensors(u64),
    TooManyKeys(u64),
    StringTooLong {
        at: u64,
        len: u64,
    },
    ArrayTooLong {
        at: u64,
        len: u64,
    },
    /// An array of arrays: GGUF writers produce none, and a reader that
    /// follows them recurses as deep as the file says.
    NestedArray {
        at: u64,
    },
    UnknownValueType {
        at: u64,
        ty: u32,
    },
    /// A metadata key this reader relies on holds the wrong type.
    KeyType {
        at: u64,
        ty: u32,
    },
    /// general.alignment is zero, not a power of two, or above 1 MiB.
    BadAlignment(u32),
    /// Tensor `tensor`, counted from 0 in header order, broke a bound.
    Tensor {
        tensor: u64,
        why: TensorError,
    },
}
