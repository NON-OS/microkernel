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

//! The History page's list of entries, or its note when there are none.

use alloc::format;
use nonos_app_skeleton::PaintBuffer;

use super::history_labels::{kind, stage};
use crate::wallet::etna::parts::tile::{row_height, tile, tile_row};
use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::tokens::TEXT_3;
use crate::wallet::etna::wrap::wrapped;
use crate::wallet::etna::Role;
use crate::wallet::state::shield_log::Entry;

const EMPTY: &str = "No shielded activity yet. Deposits, payments, withdrawals and payments \
     received appear here with the state each one has reached, kept in this account's \
     shield store.";

/* Draws the entries from `y` down and returns the y below them. */
pub(super) fn rows(fb: &mut PaintBuffer, c: Rect, mut y: u32, log: &[Entry]) -> u32 {
    if log.is_empty() {
        y += wrapped(fb, c.x as i32, y as i32, c.w as i32, Role::Lead, EMPTY, TEXT_3) as u32;
    } else {
        let at = Rect::new(c.x, y, c.w, row_height() * log.len() as u32);
        tile(fb, at);
        for (i, e) in log.iter().enumerate() {
            let name = format!("{} {} {}", kind(e.kind), e.amount, super::consts::ticker(e.asset));
            let tx =
                e.tx.as_ref()
                    .filter(|t| t.len() > 10)
                    .map(|t| format!("{}  {}..", stage(e.stage), &t[..10]));
            let value = tx.unwrap_or_else(|| alloc::string::String::from(stage(e.stage)));
            tile_row(fb, at, y + row_height() * i as u32, &name, &value, i + 1 == log.len());
        }
        y += at.h;
    }
    y
}
