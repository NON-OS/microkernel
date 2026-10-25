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
//! The stake amount and lock term, or the positions offered to close.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use nonos_app_skeleton::PaintBuffer;

use super::super::hits::{self, Press};
use crate::wallet::etna::parts::fact::fact;
use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::tokens::GAP;
use crate::wallet::nox::{lock_days, LOCK_TERMS};
use crate::wallet::screen::shield::parts::{chips, field};
use crate::wallet::state::State;

/* Positions offered to close at once; the rest are said, not hidden. */
const SHOWN: u64 = 8;

/// The form under the tabs; takes the running `y` and returns it.
pub(super) fn form(fb: &mut PaintBuffer, c: Rect, mut y: u32, state: &State) -> u32 {
    if state.stake.unstake == 0 {
        let (at, h) = field(fb, c, y, "AMOUNT, NOX", &super::text::typed(state), "0.0", true);
        hits::put(Press::Field(0), at);
        y += h + GAP / 2;
        let terms: Vec<String> = LOCK_TERMS
            .iter()
            .map(
                |(s, _)| {
                    if *s == 0 {
                        String::from("no lock")
                    } else {
                        format!("{} d", lock_days(*s))
                    }
                },
            )
            .collect();
        let refs: Vec<&str> = terms.iter().map(String::as_str).collect();
        y += chips(fb, c, y, &refs, Some(state.stake.lock), Press::Term) + GAP;
    } else {
        let n = if state.nox.positions_ready { state.nox.positions.min(SHOWN) } else { 0 };
        let names: Vec<String> = (0..n).map(|i| format!("#{i}")).collect();
        let refs: Vec<&str> = names.iter().map(String::as_str).collect();
        if !state.nox.positions_ready {
            y += fact(fb, c.x, y, c.w, "positions", super::super::amounts::UNREAD);
        } else if refs.is_empty() {
            y += fact(fb, c.x, y, c.w, "positions", "none open");
        } else {
            y += chips(fb, c, y, &refs, Some(state.stake.position as u8), Press::Pick) + GAP;
            if state.nox.positions > SHOWN {
                let more = format!("the first {SHOWN} of {}", state.nox.positions);
                y += fact(fb, c.x, y, c.w, "shown", &more);
            }
        }
    }
    y
}
