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

//! A purchase's content, top to bottom: the lead, at an escrow consent
//! the escrow's terms, the rows, then the consent's detail line, or a
//! voucher channel's form.

use nonos_app_skeleton::PaintBuffer;

use super::rows::rows;
use crate::wallet::etna::parts::fact::fact;
use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::tokens::TEXT_3;
use crate::wallet::etna::wrap::wrapped;
use crate::wallet::etna::Role;
use crate::wallet::order::intake::Pending;
use crate::wallet::order::terms::lines::Lines;
use crate::wallet::screen::page::lead;

/// Paint it in `c`; returns where the content ends.
pub fn paint(p: &Pending, consent: Option<&Lines>, fb: &mut PaintBuffer, c: Rect, s: &str) -> u32 {
    let mut y = lead(fb, c, s);
    if let Some(l) = consent {
        y = lead(fb, Rect { y, ..c }, &l.escrow);
    }
    for (label, value) in rows(p) {
        y += fact(fb, c.x, y, c.w, label, &value);
    }
    if let Some(l) = consent {
        y += wrapped(fb, c.x as i32, y as i32, c.w as i32, Role::Fact, &l.detail, TEXT_3) as u32;
    }
    if super::form::open_form(p) {
        y += super::form::paint(p, fb, c, y);
    }
    y
}
