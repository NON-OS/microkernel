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

//! The Proving screen's last spend, or its note when nothing is running.

use alloc::format;
use nonos_app_skeleton::PaintBuffer;

use crate::wallet::etna::parts::fact::fact;
use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::tokens::{BAD, GAP, TEXT_3};
use crate::wallet::etna::wrap::wrapped;
use crate::wallet::etna::Role;
use crate::wallet::state::shield_ui::ShieldUi;

const IDLE: &str = "Nothing is being proved right now. A proof starts when you confirm a \
     private payment or a withdrawal, and its progress and time taken are shown while it runs.";

/* Draws the last spend from `y` down and returns the y below it. */
pub(super) fn spend(fb: &mut PaintBuffer, ui: &ShieldUi, c: Rect, mut y: u32) -> u32 {
    let (x, w) = (c.x as i32, c.w as i32);
    match &ui.spend {
        Some(s) => {
            let what = super::history_labels::kind(s.kind);
            y += fact(
                fb,
                c.x,
                y,
                c.w,
                "payment",
                &format!("{what} {} {}", s.amount, super::consts::ticker(s.asset)),
            );
            y += fact(fb, c.x, y, c.w, "state", &s.state);
            if s.published {
                y += fact(fb, c.x, y, c.w, "since handed off", &format!("{} min", s.minutes));
            }
            if let Some(tx) = s.tx.as_ref().filter(|t| !t.is_empty()) {
                y += fact(fb, c.x, y, c.w, "transaction", &super::review_text::short(tx));
            }
            for weak in &s.weakened {
                y += wrapped(fb, x, y as i32, w, Role::Lead, weak, BAD) as u32;
            }
            if let Some(why) = &s.refusal {
                let text = format!("The lander said: {why}");
                y += wrapped(fb, x, y as i32, w, Role::Lead, &text, BAD) as u32;
            }
            if s.self_settle && s.tx.is_none() {
                y += GAP / 2;
                y += wrapped(fb, x, y as i32, w, Role::Lead, super::review_rows::LINKS, TEXT_3)
                    as u32;
            }
        }
        None if ui.waiting.is_none() && ui.earlier.is_empty() => {
            y += wrapped(fb, x, y as i32, w, Role::Lead, IDLE, TEXT_3) as u32;
        }
        None => {}
    }
    y
}
