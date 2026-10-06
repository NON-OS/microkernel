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

//! What a tensor's description may not do.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TensorError {
    /// Dimensions other than 1 to 4, ggml's GGML_MAX_DIMS.
    Dims(u32),
    ZeroDimension,
    DimensionTooLarge(u64),
    UnknownType(u32),
    /// The first dimension is not a whole number of the type's blocks.
    RowNotWholeBlocks {
        ne0: u64,
        block: u64,
    },
    /// The element count or byte size passes the limit.
    TooLarge,
    /// The data offset is not a multiple of the alignment.
    Misaligned(u64),
    /// The data runs past the end of the file.
    PastEnd,
    /// The data shares bytes with another tensor's.
    Overlap,
}
