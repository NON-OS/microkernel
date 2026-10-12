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

//! Leave the firmware framebuffer for a display driver once one is up.
//!
//! The compositor takes GOP when no driver has announced itself within its
//! first 500 ms. On a multi-core machine that wait can end before init has
//! even spawned the driver, and once a driver talks to its device the
//! device may stop scanning out the firmware framebuffer, so a compositor
//! that stays on GOP draws to a buffer nobody shows. While on GOP the
//! server loop asks one name of the display list per tick, on Backoff's
//! schedule; the first that answers takes the display.

use crate::display::Backoff;
use crate::state::Context;

use super::discover::{displays::DISPLAYS, lookup_at};

pub fn upgrade_from_gop(ctx: &mut Context, backoff: &mut Backoff, now_ms: i64) -> bool {
    if !ctx.gop_mode {
        return false;
    }
    let Some(ep) = backoff.due(now_ms, DISPLAYS.len()).and_then(lookup_at) else {
        return false;
    };
    let Ok(next) = super::prime_wire::prime(ep) else {
        return false;
    };
    super::adopt::adopt(ctx, next);
    true
}
