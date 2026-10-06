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

//! A tensor's size in bytes from its shape and type, every product checked.

use crate::error::GgufError;
use crate::limits::Limits;
use crate::tensor_error::TensorError;
use crate::types::block_of;

pub(crate) fn tensor_bytes(shape: &[u64], ty: u32, t: u64, lim: &Limits) -> Result<u64, GgufError> {
    let (block, block_bytes) =
        block_of(ty).ok_or(GgufError::Tensor { tensor: t, why: TensorError::UnknownType(ty) })?;
    if shape[0] % block != 0 {
        return Err(GgufError::Tensor {
            tensor: t,
            why: TensorError::RowNotWholeBlocks { ne0: shape[0], block },
        });
    }
    let too_large = GgufError::Tensor { tensor: t, why: TensorError::TooLarge };
    let mut elements = 1u64;
    for d in shape {
        elements = elements.checked_mul(*d).ok_or(too_large)?;
    }
    let bytes = (elements / block).checked_mul(block_bytes).ok_or(too_large)?;
    if bytes > lim.max_tensor_bytes {
        return Err(too_large);
    }
    Ok(bytes)
}
