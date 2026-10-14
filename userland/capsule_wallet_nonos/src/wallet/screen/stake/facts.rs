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
//! What the account holds and earns, and the contract's figures.

use alloc::format;
use alloc::string::String;

use nonos_app_skeleton::PaintBuffer;

use super::text::word;
use crate::wallet::etna::parts::fact::fact;
use crate::wallet::etna::rect::Rect;
use crate::wallet::nox::format_apr;
use crate::wallet::state::State;

/// The facts under the form; takes the running `y` and returns it.
pub(super) fn facts(fb: &mut PaintBuffer, c: Rect, mut y: u32, state: &State) -> u32 {
    y += fact(
        fb,
        c.x,
        y,
        c.w,
        "this account holds",
        &word(state.nox.balance_ready, &state.nox.balance_wei),
    );
    y += fact(
        fb,
        c.x,
        y,
        c.w,
        "rewards to claim",
        &word(state.nox.claimable_ready, &state.nox.claimable_wei),
    );
    let positions = if state.nox.positions_ready {
        format!("{}", state.nox.positions)
    } else {
        String::from(super::super::amounts::UNREAD)
    };
    y += fact(fb, c.x, y, c.w, "open positions", &positions);
    let apr = if state.nox.apr_ready {
        let mut b = [0u8; 24];
        let n = format_apr(state.nox.apr_bps, &mut b);
        String::from(core::str::from_utf8(&b[..n]).unwrap_or("?"))
    } else {
        String::from(super::super::amounts::UNREAD)
    };
    y += fact(fb, c.x, y, c.w, "rate now", &apr);
    y += fact(
        fb,
        c.x,
        y,
        c.w,
        "total staked",
        &word(state.nox.stats_ready, &state.nox.total_staked_wei),
    );
    y
}
