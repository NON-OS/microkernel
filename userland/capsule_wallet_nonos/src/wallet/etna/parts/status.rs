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

//! The machine's own line at the foot of every screen, for example
//! "Sepolia  ·  synced  ·  Tor", in mono 10.

use alloc::string::String;

use nonos_app_skeleton::PaintBuffer;

use super::super::roles::Role;
use super::super::text::{draw, line};
use super::super::tokens::{SIDE, STATUS_H};

pub fn status_line(fb: &mut PaintBuffer, x: u32, y: u32, parts: &[&str]) {
    let joined: String = parts.join("  \u{b7}  ");
    let top = y + (STATUS_H - line(Role::Status) as u32) / 2;
    draw(fb, (x + SIDE) as i32, top as i32, Role::Status, &joined);
}
