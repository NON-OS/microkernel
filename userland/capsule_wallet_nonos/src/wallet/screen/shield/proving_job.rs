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

//! What the shield service is doing, and the running job on the Proving screen.

use alloc::format;
use nonos_app_skeleton::PaintBuffer;
use shield_wire::*;

use crate::wallet::etna::parts::fact::fact;
use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::tokens::{CYAN, GAP, LINE_2, TEXT_3};
use crate::wallet::etna::wrap::wrapped;
use crate::wallet::etna::Role;
use crate::wallet::state::shield_ui::ShieldUi;

const PROVING: &str = "The proof is being built on this machine. Nothing has left it yet.";

pub fn doing(op: u16) -> &'static str {
    match op {
        OP_SEND => "proving a private payment",
        OP_WITHDRAW => "proving a withdrawal",
        OP_OPEN_WORDS | OP_OPEN_KEY => "opening the shield",
        OP_SYNC => "reading the pool",
        OP_REVIEW_SHIELD | OP_REVIEW_SELF_SETTLE => "checking with the pool",
        OP_QUOTE => "reading the fee",
        OP_CONFIRM => "sending",
        OP_FOLLOW => "looking for the payment on chain",
        OP_TAKE_BACK => "taking the notes back",
        _ => "working",
    }
}

/* Draws the running job from `y` down and returns the y below it. */
pub(super) fn job(fb: &mut PaintBuffer, ui: &ShieldUi, c: Rect, mut y: u32) -> u32 {
    let (x, w) = (c.x as i32, c.w as i32);
    if let Some(job) = ui.waiting {
        if matches!(job.op, OP_SEND | OP_WITHDRAW) {
            y += wrapped(fb, x, y as i32, w, Role::Lead, PROVING, TEXT_3) as u32 + GAP;
        }
        y += fact(fb, c.x, y, c.w, "now", doing(job.op));
        let (m, s) = (ui.job.elapsed_s / 60, ui.job.elapsed_s % 60);
        y += fact(fb, c.x, y, c.w, "elapsed", &format!("{m} min {s:02} s"));
        if let Some(phase) = &ui.job.phase {
            y += fact(fb, c.x, y, c.w, "phase", phase);
        }
        if let Some(p) = ui.job.permille {
            fb.fill_round(c.x, y + GAP / 2, c.w, 6, 3, LINE_2);
            let lit = (c.w as u64 * p as u64 / 1000) as u32;
            fb.fill_round(c.x, y + GAP / 2, lit.max(1), 6, 3, CYAN);
            y += GAP + 6;
        }
        y += GAP;
    }
    y
}
