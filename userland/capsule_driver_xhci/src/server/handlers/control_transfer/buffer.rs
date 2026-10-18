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
use crate::dma::DmaRegion;
use crate::error::XhciResult;
use crate::server::context::Context;

/// The data stage's buffer, zeroed; none for a request without one.
pub(super) fn data_buffer(ctx: &Context, len: u16) -> XhciResult<Option<DmaRegion>> {
    if len == 0 {
        return Ok(None);
    }
    let region = ctx.driver.dma_pool.alloc(len as u64)?;
    region.zero();
    Ok(Some(region))
}
