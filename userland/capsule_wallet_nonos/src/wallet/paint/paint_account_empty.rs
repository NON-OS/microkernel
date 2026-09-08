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

//! The screen a wallet shows before it is a wallet.
//!
//! This was the worst surface in the application and the first one every new
//! reader sees. With no account, every figure on it resolved to an em-dash, so
//! the largest thing on screen was a punctuation mark, and nothing anywhere said
//! how to fix that. The keys to generate or import an account existed and worked;
//! they were simply never mentioned.
//!
//! An empty state is not a degraded full state. It has one job, which is to make
//! the next step obvious, so it says what is missing, what will happen, and
//! offers the two ways forward as actual buttons.

use nonos_app_skeleton::PaintBuffer;

use super::scale;
use super::ui;
use crate::wallet::theme::{DIM, FG, MUTED};

const PAD: u32 = 24;
const TITLE_DROP: u32 = 40;
const BODY_DROP: u32 = 30;
const BTN_W: u32 = 190;
const BTN_H: u32 = 42;

pub fn paint_account_empty(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, h: u32) {
    ui::card(fb, x, y, w, h);
    let ix = x + PAD;

    let _ = fb.text_ttf(ix as i32, (y + PAD) as i32, "NO ACCOUNT YET", DIM(), scale::LABEL);
    let title_y = y + PAD + TITLE_DROP;
    let _ = fb.text_ttf(ix as i32, title_y as i32, "Create your wallet", FG(), scale::TITLE);

    // One line, not two. Two lines put the second at y+132 with the buttons
    // anchored at y+134, so the copy ran straight through them: the card is only
    // as tall as the network card beside it and there is room for one.
    let body_y = title_y + BODY_DROP + 8;
    let _ = fb.text_ttf(
        ix as i32,
        body_y as i32,
        "Generated on this machine, sealed to it, never uploaded.",
        MUTED(),
        scale::BODY,
    );

    // Anchored to the bottom of the card so the two cards on this row end level
    // whatever the copy above does.
    let btn_y = y + h.saturating_sub(PAD + BTN_H);
    ui::primary(fb, ix, btn_y, BTN_W, b"Create wallet");
    ui::outline(fb, ix + BTN_W + 14, btn_y, BTN_W, b"Import existing");
    let hint_x = ix + (BTN_W + 14) * 2 + 16;
    let _ = fb.text_ttf(hint_x as i32, (btn_y + 12) as i32, "G  or  I", DIM(), scale::BODY);
}
