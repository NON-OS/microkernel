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
 * The typed parts of a private payment: the receiver, the amount, and the
 * override for spending a note before its wait is over, which appears only
 * while that wait is unmet and says what spending early costs.
 */

use nonos_app_skeleton::PaintBuffer;

use super::consts::{ticker, OVERRIDE};
use super::field::field;
use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::tokens::{GAP, TEXT_3};
use crate::wallet::etna::wrap::wrapped;
use crate::wallet::etna::Role;
use crate::wallet::screen::hits::{self, Press};
use crate::wallet::state::shield_ui::{ShieldUi, FIELD_AMOUNT, FIELD_OVERRIDE, FIELD_TO};

const EARLY: &str = "This note has not waited long enough. Spending it now makes this \
     easier to link to its deposit. To go ahead anyway, type";

pub fn fields(fb: &mut PaintBuffer, c: Rect, y: u32, ui: &ShieldUi) -> u32 {
    let (to_at, a) =
        field(fb, c, y, "TO", &ui.to, "nox1... (paste with Ctrl+V)", ui.focus == FIELD_TO);
    hits::put(Press::Field(FIELD_TO), to_at);
    let name = alloc::format!("AMOUNT, {}", ticker(ui.asset));
    let focused = ui.focus == FIELD_AMOUNT;
    let (am_at, b) = field(fb, c, y + a + GAP / 2, &name, &ui.amount, "0.0", focused);
    hits::put(Press::Field(FIELD_AMOUNT), am_at);
    a + GAP / 2 + b + GAP
}

pub fn early(fb: &mut PaintBuffer, c: Rect, y: u32, ui: &ShieldUi) -> u32 {
    if super::meter::waited(ui) {
        return 0;
    }
    let text = alloc::format!("{EARLY} \"{OVERRIDE}\".");
    let t = y + GAP / 2;
    let h = wrapped(fb, c.x as i32, t as i32, c.w as i32, Role::Lead, &text, TEXT_3) as u32;
    let focused = ui.focus == FIELD_OVERRIDE;
    let (at, f) = field(fb, c, t + h + GAP / 2, "OVERRIDE", &ui.override_text, OVERRIDE, focused);
    hits::put(Press::Field(FIELD_OVERRIDE), at);
    GAP / 2 + h + GAP / 2 + f + GAP
}
