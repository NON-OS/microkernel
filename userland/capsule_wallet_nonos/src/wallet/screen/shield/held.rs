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
 * The line under a Shield form that says why its button is grey, so a
 * holder never faces a dead button with no reason given.
 */

use nonos_app_skeleton::PaintBuffer;

use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::tokens::{GAP, TEXT_3};
use crate::wallet::etna::wrap::wrapped;
use crate::wallet::etna::Role;
use crate::wallet::state::shield_ui::ShieldUi;

/* Draws the reason at `y`, if the button is held back; returns the height used. */
pub fn held(fb: &mut PaintBuffer, c: Rect, y: u32, ui: &ShieldUi, screen: u8) -> u32 {
    let Some(why) = super::check::held_back(ui, screen) else { return 0 };
    let at = y + GAP / 2;
    GAP / 2 + wrapped(fb, c.x as i32, at as i32, c.w as i32, Role::Lead, &why, TEXT_3) as u32
}
