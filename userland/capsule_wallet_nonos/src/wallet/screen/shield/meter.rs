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

/*
 * The spend-wait meter: how far the pool has grown past the newest note and
 * how long ago it landed, each against its threshold. Both must be met. An
 * uncounted wait is said to be uncounted, never drawn as empty or full.
 */

use alloc::format;
use nonos_app_skeleton::PaintBuffer;

use super::consts::{WAIT_HOURS, WAIT_LEAVES};
use crate::wallet::etna::parts::fact::fact;
use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::text::line;
use crate::wallet::etna::tokens::{CYAN, LINE_2, TIGHT};
use crate::wallet::etna::Role;
use crate::wallet::state::shield_ui::ShieldUi;

const BAR: u32 = 4;

fn bar(fb: &mut PaintBuffer, c: Rect, y: u32, name: &str, have: u32, need: u32) -> u32 {
    let h = fact(fb, c.x, y, c.w, name, &format!("{} of {need}", have.min(need)));
    let top = y + h + TIGHT / 2;
    fb.fill_round(c.x, top, c.w, BAR, BAR / 2, LINE_2);
    let lit = c.w * have.min(need) / need.max(1);
    if lit > 0 {
        fb.fill_round(c.x, top, lit, BAR, BAR / 2, CYAN);
    }
    h + TIGHT / 2 + BAR + TIGHT
}

/* Whether the newest note may be spent without the typed override. */
pub fn waited(ui: &ShieldUi) -> bool {
    matches!((ui.leaves_since, ui.hours_since), (Some(l), Some(h)) if l >= WAIT_LEAVES && h >= WAIT_HOURS)
}

pub fn meter(fb: &mut PaintBuffer, c: Rect, y: u32, ui: &ShieldUi) -> u32 {
    match (ui.leaves_since, ui.hours_since) {
        (Some(l), Some(h)) => {
            let a = bar(fb, c, y, "Pool growth, leaves", l, WAIT_LEAVES);
            a + bar(fb, c, y + a, "Time since deposit, hours", h, WAIT_HOURS)
        }
        _ => {
            let h = fact(fb, c.x, y, c.w, "Spend wait", "not counted yet");
            h + line(Role::Fact) as u32 / 2
        }
    }
}
