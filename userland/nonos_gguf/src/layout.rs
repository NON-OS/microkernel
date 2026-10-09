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

//! Where the tensors' bytes lie: the data section starts at the first
//! aligned offset after the header, and every tensor must start aligned,
//! end inside the file, and share no byte with another.

use alloc::vec::Vec;

use crate::error::GgufError;
use crate::tensor::Tensor;
use crate::tensor_error::TensorError;

/// The first multiple of `align` at or after `pos`. `align` is a power of two.
pub(crate) fn align_up(pos: u64, align: u64) -> Option<u64> {
    Some(pos.checked_add(align - 1)? & !(align - 1))
}

/// Check every tensor against the file; returns the bytes they cover.
pub(crate) fn check_layout(
    tensors: &[Tensor],
    data_start: u64,
    align: u64,
    len: u64,
) -> Result<u64, GgufError> {
    let mut spans: Vec<(u64, u64, u64)> = Vec::with_capacity(tensors.len());
    for (t, x) in tensors.iter().enumerate() {
        let t = t as u64;
        if x.offset % align != 0 {
            return Err(GgufError::Tensor { tensor: t, why: TensorError::Misaligned(x.offset) });
        }
        let start = data_start
            .checked_add(x.offset)
            .ok_or(GgufError::Tensor { tensor: t, why: TensorError::PastEnd })?;
        let end = start
            .checked_add(x.bytes)
            .ok_or(GgufError::Tensor { tensor: t, why: TensorError::PastEnd })?;
        if end > len {
            return Err(GgufError::Tensor { tensor: t, why: TensorError::PastEnd });
        }
        spans.push((start, end, t));
    }
    spans.sort_unstable();
    let mut covered = 0u64;
    for pair in spans.windows(2) {
        if pair[1].0 < pair[0].1 {
            return Err(GgufError::Tensor { tensor: pair[1].2, why: TensorError::Overlap });
        }
    }
    for (start, end, _) in &spans {
        /*
         * Disjoint spans inside a file of `len` bytes sum to at most `len`.
         */
        covered += end - start;
    }
    Ok(covered)
}
