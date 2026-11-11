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

use nonos_app_skeleton::PaintBuffer;
use nonos_policy_proto::Field;
use nonos_toolkit::icons::{draw, IconId};
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::text;

use crate::settings::state::FieldValue;

use super::control_value::choice_text;
use super::metrics::BODY_PX;

/// Side of the square the disclosure chevron after an inline choice is drawn
/// in. The mark spans about nine pixels of it, its right end at `right`.
const CHEVRON: u32 = 15;

pub fn paint_plain(fb: &mut PaintBuffer, s: &str, right: u32, screen_y: i32, row_h: u32) {
    let top = text::centre_top(0, row_h, BODY_PX) + screen_y;
    text::right(fb, right, top, s, roles().text_2, BODY_PX);
}

pub fn paint_bool_text(
    fb: &mut PaintBuffer,
    value: FieldValue,
    right: u32,
    screen_y: i32,
    row_h: u32,
) {
    let label = match value {
        FieldValue::Bool(true) => "Yes",
        FieldValue::Bool(false) => "No",
        _ => "Unknown",
    };
    paint_plain(fb, label, right, screen_y, row_h);
}

pub fn paint_choice(
    fb: &mut PaintBuffer,
    field: Field,
    value: FieldValue,
    right: u32,
    screen_y: i32,
    row_h: u32,
) {
    let top = screen_y + (row_h / 2) as i32 - (CHEVRON / 2) as i32;
    if let Ok(y) = u32::try_from(top) {
        let x = right - CHEVRON + CHEVRON / 5;
        draw(fb, IconId::ChevronDown, x, y, CHEVRON, roles().muted.argb());
    }
    match choice_text(field, value) {
        Some(v) => paint_plain(fb, v.as_str(), right - 17, screen_y, row_h),
        None => paint_plain(fb, "--", right - 17, screen_y, row_h),
    }
}

pub use super::control_field::{paint_number, paint_string};
