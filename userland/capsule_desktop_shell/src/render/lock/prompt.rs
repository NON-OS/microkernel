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

//! The prompt that chooses or changes the passphrase, on the first output,
//! over everything else the chrome draws.

use super::super::dialog_kit::{border, button, fill, margin, BORDER, CANCEL_BG, DIM, FG, PANEL};
use super::super::dialog_text::{line, wrap_at};
use crate::render::ui_font::{line_h, px, TITLE_PX, UI_PX};
use crate::state::lock::choose::ChooseNote;
use crate::state::lock::geometry::prompt;
use crate::state::lock::{words, ChooseStep, Purpose};
use crate::state::Context;
use nonos_lock::rules::Refusal;

const SAVE_BG: u32 = 0xFF1E_4A7A;
const WARN: u32 = 0xFFE8_B75C;

pub fn paint(ctx: &Context) {
    let Some(c) = ctx.lock.choose.as_ref() else { return };
    let out = ctx.lock.outputs.first().copied().unwrap_or((0, 0, ctx.width, ctx.height));
    let p = prompt(out, px(1));
    let panel = super::rect(p.panel);
    fill(ctx, panel, PANEL);
    border(ctx, panel, BORDER);
    let (x, max_w) = (panel.x + margin(), panel.width.saturating_sub(margin() * 2));
    let mut y = panel.y + margin();
    let title = match c.purpose() {
        Purpose::First => words::CHOOSE_FIRST,
        Purpose::Change => words::CHOOSE_CHANGE,
    };
    line(ctx, x, y, title, FG, TITLE_PX, max_w);
    y += line_h(TITLE_PX) + line_h(UI_PX) / 3;
    let mut rest = words::CANNOT_RECOVER;
    let label_y = p.field.1.saturating_sub(line_h(UI_PX) + px(6));
    while !rest.is_empty() && y + line_h(UI_PX) <= label_y {
        let end = wrap_at(rest, UI_PX, max_w);
        line(ctx, x, y, &rest[..end], DIM, UI_PX, max_w);
        rest = rest[end..].trim_start();
        y += line_h(UI_PX);
    }
    let (label, chars) = match c.step() {
        ChooseStep::Current => (words::ASK_CURRENT, c.current.chars()),
        ChooseStep::New => (words::ASK_NEW, c.new.chars()),
        ChooseStep::Again | ChooseStep::Saving => (words::ASK_AGAIN, c.again.chars()),
    };
    line(ctx, x, label_y, label, FG, UI_PX, max_w);
    let saving = c.step() == ChooseStep::Saving;
    super::field::paint(ctx, p.field, chars, "", !saving);
    if let Some(note) = note(c.note()) {
        let ny = p.field.1 + p.field.3 + px(6);
        line(ctx, x, ny, note, WARN, UI_PX, max_w);
    }
    let primary = match c.step() {
        ChooseStep::Current | ChooseStep::New => words::NEXT,
        ChooseStep::Again => words::SAVE,
        ChooseStep::Saving => words::SAVING,
    };
    let secondary = match c.purpose() {
        Purpose::First => words::NOT_NOW,
        Purpose::Change => words::CANCEL,
    };
    button(ctx, super::rect(p.primary), SAVE_BG, primary, true);
    button(ctx, super::rect(p.secondary), CANCEL_BG, secondary, false);
}

fn note(n: ChooseNote) -> Option<&'static str> {
    Some(match n {
        ChooseNote::None => return None,
        ChooseNote::Refused(Refusal::Short) => words::TOO_SHORT,
        ChooseNote::Refused(Refusal::Long) => words::TOO_LONG,
        ChooseNote::Refused(Refusal::NotText) => words::NOT_TEXT,
        ChooseNote::Refused(Refusal::Mismatch) => words::MISMATCH,
        ChooseNote::Wrong => words::NOT_CURRENT,
        ChooseNote::Wait => words::WAIT_THEN,
        ChooseNote::Failed => words::NOT_SAVED,
    })
}
