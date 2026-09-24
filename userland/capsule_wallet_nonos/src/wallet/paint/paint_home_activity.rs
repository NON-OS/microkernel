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

use super::scale;

// A caption line and the clearance a 17px line box needs under it.
const ROW_DROP: u32 = 26;
use nonos_app_skeleton::PaintBuffer;

use super::ui;
use crate::wallet::state::State;
use crate::wallet::theme::{ACCENT, DIM, FG, LINE2, MUTED};

/// `y` is the section's caption row. It used to be a literal here and another
/// literal in the caller, so moving anything above this section left a gap
/// nobody could see the cause of.
pub fn paint_home_activity(state: &State, fb: &mut PaintBuffer, cx: u32, cw: u32, y: u32) {
    let col = (cw - 28) / 2;
    let rx = cx + col + 28;
    let _ = fb.text_ttf(cx as i32, y as i32, "ENABLED RAILS", DIM(), scale::BODY);
    let rows = y + ROW_DROP;

    // ETH and NOX show live balances; a field still loading shows "fetching"
    // while the link is up, a dash only when there is no route.
    let up = state.net.rpc_chain_ok;
    let pending = if up { "\u{2026}" } else { "\u{2014}" };
    let mut buf = [0u8; 40];
    let eth = if state.balance_ready {
        let n = super::format_eth::format_eth(
            super::format_eth::lower_u64(&state.balance_wei),
            &mut buf,
        );
        core::str::from_utf8(&buf[..n]).unwrap_or(pending)
    } else {
        pending
    };
    rail(fb, cx, rows, col, "ETH", b"L1", eth);
    let mut nox_b = [0u8; 48];
    let nox = crate::wallet::nox::live_amount(
        state.nox.balance_ready,
        &state.nox.balance_wei,
        up,
        &mut nox_b,
    );
    rail(fb, cx, 570, col, "NOX", b"ERC-20", nox);
    rail(fb, cx, 632, col, "PR", b"RSVD", "\u{2014}");

    super::home_activity_row::activity(state, fb, rx, col, rows);
}

fn rail(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, sym: &str, tag: &[u8], val: &str) {
    ui::card(fb, x, y, w, 54);
    fb.fill_rect(x, y, 3, 54, ACCENT());
    let sx = fb.text_ttf((x + 18) as i32, (y + 18) as i32, sym, FG(), scale::VALUE);
    ui::badge(fb, sx as u32 + 10, y + 18, tag, LINE2(), MUTED());
    let vw = fb.measure_ttf(val, scale::VALUE).max(0) as u32;
    let _ = fb.text_ttf((x + w - 18 - vw) as i32, (y + 17) as i32, val, FG(), scale::VALUE);
}
