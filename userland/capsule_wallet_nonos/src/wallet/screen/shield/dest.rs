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
 * Where a withdrawal goes: a fresh address of this wallet, or this
 * machine's meter (B7). The meter's address comes from the meter itself at
 * review; nothing here is typed.
 */

use nonos_app_skeleton::PaintBuffer;

use super::chips::chips;
use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::tokens::GAP;
use crate::wallet::screen::hits::Press;
use crate::wallet::state::shield_ui::ShieldUi;

pub fn dest(fb: &mut PaintBuffer, c: Rect, y: u32, ui: &ShieldUi) -> u32 {
    let h = super::pick::label(fb, c, y, "TO");
    let picked = Some(ui.payee.to_meter as u8);
    h + chips(fb, c, y + h, &["a fresh address of this wallet", "the meter"], picked, Press::Dest)
        + GAP
}
