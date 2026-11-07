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

//! When the screen last changed (state/presented.rs). Any client may ask:
//! it says only that a frame was shown and when.

use crate::protocol::Request;
use crate::server::respond;
use crate::state::Context;

pub fn handle(
    ctx: &Context,
    sender: u32,
    req: &Request,
    tx: &mut [u8],
) -> Result<(), &'static str> {
    respond::status_payload(sender, req, 0, &ctx.presented.encode(), tx)
}
