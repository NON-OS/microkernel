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
 * A bright background must not make VGA text blink.
 *
 * The boot attribute helpers are included by path. make_attr put the whole
 * background in bits 4 to 7, and bit 7 is blink with the attribute
 * controller's default setting, so a bright background blinked on the dark
 * one; bg_color read bit 7 as a background bit. The checks below fail against
 * that code. The arch ColorCode is covered for all colour pairs in Lean.
 */

#[path = "../../../../src/boot/vga/colors.rs"]
pub mod colors;
mod tests;
