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

//! OP_SCANOUT_PEER: the display driver a swapchain client may share its
//! buffers with, read only, so the plane can scan a full-screen opaque one
//! out. The client decides: a buffer it does not share is composed. 0 when
//! the driver cannot scan a client buffer out (no IOMMU, no band plane).

use crate::protocol::Request;
use crate::server::respond;
use crate::state::Context;

pub fn handle(
    ctx: &Context,
    sender: u32,
    req: &Request,
    tx: &mut [u8],
) -> Result<(), &'static str> {
    let pid = ctx.wire.map(|w| w.pid).filter(|&p| ctx.direct.able(p)).unwrap_or(0);
    respond::status_payload(sender, req, 0, &pid.to_le_bytes(), tx)
}
