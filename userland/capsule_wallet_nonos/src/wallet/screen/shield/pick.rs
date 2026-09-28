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
 * The asset and the standard size, the two choices a deposit and a
 * withdrawal share. The pool refuses any other amount, so there is no
 * field to type one into.
 */

use alloc::vec::Vec;
use nonos_app_skeleton::PaintBuffer;

use super::chips::chips;
use super::sizes::sizes;
use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::text::{draw_in, line};
use crate::wallet::etna::tokens::{GAP, TEXT_3, TIGHT};
use crate::wallet::etna::Role;
use crate::wallet::screen::hits::Press;
use crate::wallet::state::shield_ui::ShieldUi;

pub fn label(fb: &mut PaintBuffer, c: Rect, y: u32, text: &str) -> u32 {
    draw_in(fb, c.x as i32, y as i32, Role::Fact, text, TEXT_3);
    line(Role::Fact) as u32 + TIGHT
}

pub fn asset(fb: &mut PaintBuffer, c: Rect, y: u32, ui: &ShieldUi) -> u32 {
    let h = label(fb, c, y, "ASSET");
    h + chips(fb, c, y + h, &["ETH", "NOX"], Some(ui.asset), Press::Asset) + GAP
}

pub fn size(fb: &mut PaintBuffer, c: Rect, y: u32, ui: &ShieldUi) -> u32 {
    let h = label(fb, c, y, "STANDARD AMOUNT");
    let all = sizes(ui.asset);
    let labels: Vec<&str> = all.iter().map(|(s, _)| s.as_str()).collect();
    h + chips(fb, c, y + h, &labels, ui.size, Press::Pick) + GAP
}
