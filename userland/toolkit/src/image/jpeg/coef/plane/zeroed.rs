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

use alloc::vec::Vec;

use crate::image::types::DecodeError;

/// Bytes one block costs at keep size `k`, with or without the mask.
pub fn block_bytes(k: usize, masked: bool) -> usize {
    k * k * 2 + if masked && k < 8 { 8 } else { 0 }
}

/// A zero-filled vector, or an error when it cannot be allocated.
pub(in crate::image::jpeg::coef) fn zeroed<T: Clone + Default>(
    n: usize,
) -> Result<Vec<T>, DecodeError> {
    let mut v = Vec::new();
    v.try_reserve_exact(n).map_err(|_| DecodeError::BadDimensions)?;
    v.resize(n, T::default());
    Ok(v)
}
