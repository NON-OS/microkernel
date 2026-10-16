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

//! NCMP 0x0010 ACCESSIBILITY_GET: one output's settings as they are now, so
//! Settings shows what the screen is doing rather than what it last sent.
//! Body: output u32. Answer: filter, contrast, magnify, follow, u32 each. Only
//! desktop_shell asks (E_PERM otherwise), for Settings, as with 0x000F.

use crate::display::access_rule::encode;
use crate::protocol::{Request, E_INVAL, E_PERM};
use crate::server::respond;
use crate::server::trusted_pids::trusted_pids;
use crate::state::Context;

pub fn handle(
    ctx: &mut Context,
    sender: u32,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> Result<(), &'static str> {
    if sender == 0 || !trusted_pids().contains(&Some(sender)) {
        return respond::status(sender, req, E_PERM, tx);
    }
    let out = super::u32_at(body, 0).filter(|_| body.len() == 4).map(|o| o as usize);
    let Some(a) = out.and_then(|o| ctx.looks.access.get(o)).copied() else {
        return respond::status(sender, req, E_INVAL, tx);
    };
    respond::status_payload(sender, req, 0, &encode(&a), tx)
}
