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
 * The private receive address. The nox1 address is derived from the
 * recovery phrase by the shield service; until one has answered there is
 * no address to show, and the screen says where it will come from.
 */

use nonos_app_skeleton::PaintBuffer;

use crate::wallet::etna::parts::qr::qr;
use crate::wallet::etna::parts::quiet::quiet_group;
use crate::wallet::etna::parts::value::value_block;
use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::tokens::{GAP, TEXT_3};
use crate::wallet::etna::wrap::wrapped;
use crate::wallet::etna::Role;
use crate::wallet::screen::hits::{self, Press};
use crate::wallet::state::State;

const WAITING: &str = "Your private nox1 address appears here once the shield service has \
     derived it from your recovery phrase. Share it to be paid privately.";
/* 177 modules at version 38: two pixels a module at the least. */
const QR_SIDE: u32 = 400;
const COPY: &str = "Copy the private address";
pub const ROW_COPY: u8 = 0;

pub fn receive(state: &State, fb: &mut PaintBuffer, c: Rect, y: u32) -> u32 {
    let head = super::pick::label(fb, c, y, "RECEIVE PRIVATELY");
    let mut h = head;
    match &state.shield_ui.nox1 {
        Some(addr) => {
            if qr(fb, c.x, y + h, QR_SIDE.min(c.w), addr.as_bytes()) {
                h += QR_SIDE.min(c.w) + GAP;
            }
            /* 2,009 letters: the head and tail to recognise it, and the
             * whole address goes by the copy row. */
            h += value_block(fb, c.x, y + h, c.w, &super::review_text::short(addr));
            let mut at = [Rect::default(); 1];
            h += GAP / 2 + quiet_group(fb, c.x, y + h + GAP / 2, c.w, &[(COPY, true)], &mut at);
            hits::put(Press::Row(ROW_COPY), at[0]);
        }
        None => {
            let t = (y + h) as i32;
            h += wrapped(fb, c.x as i32, t, c.w as i32, Role::Lead, WAITING, TEXT_3) as u32;
        }
    }
    h
}
