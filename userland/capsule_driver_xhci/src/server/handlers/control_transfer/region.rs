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

//! The DMA memory a control transfer's data stage moves through.

use crate::dma::{DmaPool, DmaRegion};
use crate::error::XhciResult;

/// `w_len` bytes, zeroed; none for a request without a data stage.
pub(super) fn data_region(pool: &DmaPool, w_len: u16) -> XhciResult<Option<DmaRegion>> {
    if w_len == 0 {
        return Ok(None);
    }
    let r = pool.alloc(w_len as u64)?;
    r.zero();
    Ok(Some(r))
}
