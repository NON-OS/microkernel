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
use super::route::route;
use crate::protocol::{Request, E_NODEV, OP_CONTROLLER_STATUS, OP_HEALTHCHECK, OP_PORT_STATUS};
use crate::server::{context::Context, error::reply_with_status};

/// Serve one request. Whether the controller failed is looked at before and
/// after each. A failed one is brought back before a request that needs
/// it, and while it cannot be every such request is refused with E_NODEV;
/// status requests are answered either way.
pub fn dispatch(ctx: &mut Context, req: &Request, body: &[u8], tx: &mut [u8]) {
    let status = matches!(req.op, OP_HEALTHCHECK | OP_CONTROLLER_STATUS | OP_PORT_STATUS);
    if ctx.driver.failed() {
        ctx.driver.dead = true;
    }
    if ctx.driver.dead && !status && ctx.driver.revive().is_err() {
        return reply_with_status(tx, req, E_NODEV);
    }
    route(ctx, req, body, tx);
    if ctx.driver.failed() {
        ctx.driver.dead = true;
    }
}
