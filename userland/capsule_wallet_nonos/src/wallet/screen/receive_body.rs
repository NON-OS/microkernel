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

//! The receive screen's body: the project, then the QR and address shown.

use nonos_app_skeleton::PaintBuffer;

use super::receive_words::*;
use crate::wallet::etna::parts::qr::qr;
use crate::wallet::etna::parts::value::value_block;
use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::tokens::{GAP, TEXT_3};
use crate::wallet::etna::wrap::wrapped;
use crate::wallet::etna::Role;
use crate::wallet::state::State;

pub(super) fn body(
    state: &State,
    fb: &mut PaintBuffer,
    c: Rect,
    private: bool,
    here: bool,
    nox1: Option<&str>,
) -> u32 {
    let lead = match (private, here) {
        (true, _) => PRIVATE,
        (false, true) => PUBLIC,
        (false, false) => PUBLIC_HERE_ONLY,
    };
    let mut y = c.y
        + wrapped(fb, c.x as i32, c.y as i32, c.w as i32, Role::Lead, lead, TEXT_3) as u32
        + GAP;
    match (private, nox1) {
        (true, Some(addr)) => {
            let side = QR_PRIVATE.min(c.w);
            if qr(fb, c.x, y, side, addr.as_bytes()) {
                y += side + GAP;
            }
            y += value_block(fb, c.x, y, c.w, addr) + GAP;
            if state.shield_ui.link.live {
                y += wrapped(fb, c.x as i32, y as i32, c.w as i32, Role::Lead, LIVE, TEXT_3) as u32
                    + GAP;
            }
        }
        (true, None) => {
            let (text, colour) = waiting_text(state);
            y += wrapped(fb, c.x as i32, y as i32, c.w as i32, Role::Lead, &text, colour) as u32
                + GAP;
        }
        (false, _) if state.acct.address_ready => {
            let hex = super::receive_address::address_text(state);
            let uri = alloc::format!("ethereum:{hex}");
            if qr(fb, c.x, y, QR_SIDE, uri.as_bytes()) {
                y += QR_SIDE + GAP;
            }
            y += value_block(fb, c.x, y, c.w, &hex) + GAP;
        }
        _ => {}
    }
    y
}
