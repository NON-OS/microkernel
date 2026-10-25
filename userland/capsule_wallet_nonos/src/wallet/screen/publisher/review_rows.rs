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

//! A publisher transaction's review rows (intent::page::rows), every value
//! whole: a lone id, address or uri boxed as the payee line is, any other
//! value wrapped with its long runs grouped by eight, so nothing is cut.

use alloc::string::String;
use alloc::vec::Vec;

use nonos_app_skeleton::PaintBuffer;

use crate::wallet::etna::groups::{grouped, RUN};
use crate::wallet::etna::parts::value::value_block;
use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::text::{draw_in, line};
use crate::wallet::etna::tokens::{GAP, TEXT, TEXT_3, TIGHT};
use crate::wallet::etna::wrap::wrapped;
use crate::wallet::etna::Role;
use crate::wallet::intent::Rows;

fn spaced(v: &str) -> String {
    let long = |w: &str| w.chars().count() > 2 * RUN;
    let words =
        v.split(' ').map(|w| if long(w) { String::from(grouped(w).as_str()) } else { w.into() });
    words.collect::<Vec<String>>().join(" ")
}

/// Draw `rows` from `y` down in `c`; returns their height.
pub fn draw(fb: &mut PaintBuffer, c: Rect, y: u32, rows: &Rows) -> u32 {
    let mut h = 0;
    for (name, value) in rows {
        draw_in(fb, c.x as i32, (y + h) as i32, Role::Fact, &name.to_uppercase(), TEXT_3);
        h += line(Role::Fact) as u32 + TIGHT / 2;
        let (x, top, w) = (c.x as i32, (y + h) as i32, c.w as i32);
        h += GAP / 2
            + match value.contains(' ') {
                true => wrapped(fb, x, top, w, Role::Fact, &spaced(value), TEXT).max(0) as u32,
                false => value_block(fb, c.x, y + h, c.w, value),
            };
    }
    h
}
