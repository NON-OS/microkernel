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

//! The Proving screen's earlier spends that have not landed yet.

use alloc::format;
use alloc::string::String;
use nonos_app_skeleton::PaintBuffer;

use crate::wallet::etna::parts::fact::fact;
use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::tokens::{GAP, TEXT_3};
use crate::wallet::etna::wrap::wrapped;
use crate::wallet::etna::Role;
use crate::wallet::state::shield_ui::ShieldUi;

const EARLIER: &str = "Earlier payments still on their way, each followed to the block:";

/* Draws the earlier spends from `y` down and returns the y below them. */
pub(super) fn earlier(fb: &mut PaintBuffer, ui: &ShieldUi, c: Rect, mut y: u32) -> u32 {
    let (x, w) = (c.x as i32, c.w as i32);
    /* Every spend proved before the last that has not landed yet. */
    if !ui.earlier.is_empty() {
        y += GAP;
        y += wrapped(fb, x, y as i32, w, Role::Lead, EARLIER, TEXT_3) as u32;
    }
    for s in &ui.earlier {
        let what = if s.amount.is_empty() {
            String::from("an earlier payment")
        } else {
            let kind = super::history_labels::kind(s.kind);
            format!("{kind} {} {}", s.amount, super::consts::ticker(s.asset))
        };
        let minutes = if s.minutes > 0 { format!(", {} min", s.minutes) } else { String::new() };
        y += fact(fb, c.x, y, c.w, &what, &format!("{}{minutes}", s.state));
    }
    y
}
