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

//! Did that click land on the magnifier? Answered from the same box the
//! painter drew into.

use crate::state::indicators::clock_stamp::{stamp, STAMP_LEN};
use crate::state::indicators::{battery, battery_text};
use crate::state::Context;

use super::search_box::{fitted, search_box};

pub fn search_hit(ctx: &Context, px: u32, py: u32) -> bool {
    let mut bbuf = [0u8; battery_text::LABEL_MAX];
    let blen = battery_text::label(battery::percent(), &mut bbuf);
    let mut sbuf = [b'-'; STAMP_LEN];
    let when: &[u8] = match stamp(&mut sbuf, ctx.clock_24h, ctx.tz_hours) {
        Some(n) => &sbuf[..n],
        None => b"--:--",
    };
    let btext = fitted(ctx, &bbuf[..blen], when);
    match search_box(ctx, btext, when) {
        Some((x, y, w)) => px >= x && px < x + w && py >= y && py < y + w,
        None => false,
    }
}
