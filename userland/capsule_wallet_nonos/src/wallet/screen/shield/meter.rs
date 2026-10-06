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
 * The spend wait: how many more notes must land in the pool and how many
 * more minutes must pass before every note may be spent, as the service
 * last counted them. An uncounted wait is said to be uncounted, never drawn
 * as met.
 */

use alloc::format;
use alloc::string::String;
use nonos_app_skeleton::PaintBuffer;

use crate::wallet::etna::parts::fact::fact;
use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::text::line;
use crate::wallet::etna::Role;
use crate::wallet::state::shield_ui::ShieldUi;

/* Whether every note may be spent without the typed override. */
pub fn waited(ui: &ShieldUi) -> bool {
    ui.wait_left == Some((0, 0))
}

/* A counted wait that is not over: the override is offered. */
pub fn must_wait(ui: &ShieldUi) -> bool {
    ui.wait_left.is_some() && !waited(ui)
}

pub fn meter(fb: &mut PaintBuffer, c: Rect, y: u32, ui: &ShieldUi) -> u32 {
    let text = match ui.wait_left {
        Some((0, 0)) => String::from("ready to spend"),
        Some((l, m)) => format!("{l} more notes, {m} more minutes"),
        None => String::from("not counted yet"),
    };
    let h = fact(fb, c.x, y, c.w, "spend wait", &text);
    h + line(Role::Fact) as u32 / 4
}
