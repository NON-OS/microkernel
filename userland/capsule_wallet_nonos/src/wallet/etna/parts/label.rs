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

//! The screen's number and name in the bar, "03  RECEIVE", and the lead
//! sentence a screen is allowed.

use alloc::{format, string::String};

use nonos_app_skeleton::PaintBuffer;

use super::super::roles::Role;
use super::super::text::draw;

pub fn screen_label(fb: &mut PaintBuffer, x: i32, top: i32, number: &str, title: &str) -> i32 {
    let text = if number.is_empty() { String::from(title) } else { format!("{number}  {title}") };
    draw(fb, x, top, Role::ScreenLabel, &text)
}
