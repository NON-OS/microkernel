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

//! What the wallet knows, as values instead of sentences: a name in mono
//! capitals on the left, the value on the right. Never a placeholder value.

use nonos_app_skeleton::PaintBuffer;

use super::super::roles::Role;
use super::super::text::{draw, draw_in, line, width};
use super::super::tokens::TEXT_3;

/// Facts are 10 apart.
pub const FACT_GAP: u32 = 10;

pub fn fact(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, name: &str, value: &str) -> u32 {
    let upper = name.to_uppercase();
    draw_in(fb, x as i32, y as i32, Role::Fact, &upper, TEXT_3);
    let vx = (x + w) as i32 - width(Role::Fact, value);
    draw(fb, vx, y as i32, Role::Fact, value);
    line(Role::Fact) as u32 + FACT_GAP
}
