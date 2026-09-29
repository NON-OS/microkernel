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
 * Which arms a box-drawing character has, and how heavy each is. Tables
 * that tools print with these characters only join up when the lines run
 * to the edges of the cell, which a font's glyph does not: the line gap
 * leaves a break between rows.
 */

/// An arm's weight: none, light, heavy or double.
pub const NONE: u8 = 0;
pub const LIGHT: u8 = 1;
pub const HEAVY: u8 = 2;
pub const DOUBLE: u8 = 3;

/// Up, down, left and right arms of `ch`, or None when it is not one of
/// the line characters drawn here.
pub fn arms(ch: char) -> Option<[u8; 4]> {
    let (l, h, d) = (LIGHT, HEAVY, DOUBLE);
    Some(match ch {
        '─' => [0, 0, l, l],
        '━' => [0, 0, h, h],
        '│' => [l, l, 0, 0],
        '┃' => [h, h, 0, 0],
        '┌' | '╭' => [0, l, 0, l],
        '┐' | '╮' => [0, l, l, 0],
        '└' | '╰' => [l, 0, 0, l],
        '┘' | '╯' => [l, 0, l, 0],
        '├' => [l, l, 0, l],
        '┤' => [l, l, l, 0],
        '┬' => [0, l, l, l],
        '┴' => [l, 0, l, l],
        '┼' => [l, l, l, l],
        '┏' => [0, h, 0, h],
        '┓' => [0, h, h, 0],
        '┗' => [h, 0, 0, h],
        '┛' => [h, 0, h, 0],
        '┣' => [h, h, 0, h],
        '┫' => [h, h, h, 0],
        '┳' => [0, h, h, h],
        '┻' => [h, 0, h, h],
        '╋' => [h, h, h, h],
        '═' => [0, 0, d, d],
        '║' => [d, d, 0, 0],
        '╔' => [0, d, 0, d],
        '╗' => [0, d, d, 0],
        '╚' => [d, 0, 0, d],
        '╝' => [d, 0, d, 0],
        '╠' => [d, d, 0, d],
        '╣' => [d, d, d, 0],
        '╦' => [0, d, d, d],
        '╩' => [d, 0, d, d],
        '╬' => [d, d, d, d],
        _ => return None,
    })
}
