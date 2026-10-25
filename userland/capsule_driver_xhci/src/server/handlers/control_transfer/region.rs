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

const DEVICE_TO_HOST: u8 = 0x80;

/// Whether a request's body carries the data stage its direction calls
/// for: an OUT request exactly wLength bytes, at most `max`; an IN request
/// none (its wLength is clamped to `max`, not refused).
pub fn stage_fits(bm_request_type: u8, w_length: u16, carried: usize, max: u16) -> bool {
    if bm_request_type & DEVICE_TO_HOST != 0 {
        return carried == 0;
    }
    carried == w_length as usize && w_length <= max
}

/// `w_len` bytes, zeroed, then `out` (an OUT request's data) at its start;
/// none for a request without a data stage.
pub fn data_region(pool: &DmaPool, w_len: u16, out: &[u8]) -> XhciResult<Option<DmaRegion>> {
    if w_len == 0 {
        return Ok(None);
    }
    let r = pool.alloc(w_len as u64)?;
    r.zero();
    let _ = r.copy_from(&out[..out.len().min(w_len as usize)]);
    Ok(Some(r))
}
