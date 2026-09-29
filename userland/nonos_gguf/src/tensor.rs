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

//! One tensor's description: its name, shape, type and where its data is.
//! The shape comes from the file, so every dimension is bounded before it is
//! multiplied, and the product is checked at every step.

use crate::cursor::Cursor;
use crate::error::GgufError;
use crate::limits::Limits;
use crate::source::ReadAt;
use crate::tensor_error::TensorError;
use crate::tensor_size::tensor_bytes;

/// ggml's GGML_MAX_DIMS.
pub const MAX_DIMS: u32 = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Tensor {
    pub ty: u32,
    pub bytes: u64,
    /// From the start of the data section.
    pub offset: u64,
}

pub(crate) fn read_tensor<S: ReadAt>(
    c: &mut Cursor<'_, S>,
    t: u64,
    lim: &Limits,
) -> Result<Tensor, GgufError> {
    c.skip_string(lim.max_string)?;
    let dims = c.u32()?;
    if dims == 0 || dims > MAX_DIMS {
        return Err(GgufError::Tensor { tensor: t, why: TensorError::Dims(dims) });
    }
    let mut shape = [0u64; MAX_DIMS as usize];
    for d in shape.iter_mut().take(dims as usize) {
        let dim = c.u64()?;
        if dim == 0 {
            return Err(GgufError::Tensor { tensor: t, why: TensorError::ZeroDimension });
        }
        if dim > lim.max_dim {
            return Err(GgufError::Tensor { tensor: t, why: TensorError::DimensionTooLarge(dim) });
        }
        *d = dim;
    }
    let ty = c.u32()?;
    let offset = c.u64()?;
    let bytes = tensor_bytes(&shape[..dims as usize], ty, t, lim)?;
    Ok(Tensor { ty, bytes, offset })
}
