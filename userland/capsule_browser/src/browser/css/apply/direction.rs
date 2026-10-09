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

use crate::browser::css::computed::{Computed, TextAlign};

/* direction, and text-align, whose start and end mean the edge the
 * direction reads from: both resolve at layout against the box's own
 * inherited direction, so a later dir changes where start text lines up.
 * Anything else goes on to the white-space applier. */
pub(super) fn apply_direction(c: &mut Computed, name: &str, value: &str) -> bool {
    let v = value.trim();
    match name {
        "direction" => match v {
            "rtl" => c.rtl = true,
            "ltr" => c.rtl = false,
            _ => {}
        },
        "text-align" => {
            let a = match v {
                "left" => TextAlign::Left,
                "right" => TextAlign::Right,
                "center" => TextAlign::Center,
                /* Justification is not done; its lines sit at the start. */
                "start" | "justify" => TextAlign::Start,
                "end" => TextAlign::End,
                _ => return true,
            };
            c.text_align = a;
        }
        _ => return super::white_space::apply_white_space(c, name, value),
    }
    true
}
