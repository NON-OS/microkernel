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

// The chrome takes the house palette. These were a private set before, close
// enough to the rest of the desktop to look intentional and different enough
// to look wrong beside it: the ground was two steps lighter and the accent was
// green while every other window was cyan.
pub const BACKGROUND: u32 = 0xFF0B1319;
pub const FOREGROUND: u32 = 0xFFE4ECF5;
pub const MUTED: u32 = 0xFF9BB0C7;
pub const HEADER_BG: u32 = 0xFF08111D;
pub const ACCENT: u32 = 0xFF35C4E2;
pub const LINE: u32 = 0xFF233243;

/// Selection follows the accent, because selection is the system saying which
/// row you are on, and that is what the accent is for.
pub const SELECTED: u32 = ACCENT;

// Content colours, which are about what a row *is* rather than about chrome,
// so they stay their own. A directory reads green here the same way it does in
// every file listing anyone has used.
pub const DIRECTORY: u32 = 0xFF6CE08C;
pub const FILE_C: u32 = 0xFFB7C4D8;

// Row banding and the selected row's ground, lifted from the background rather
// than picked, so they stay correct if the ground moves.
pub const ALT_ROW: u32 = 0xFF111A22;
pub const SELECT_BG: u32 = 0xFF17303B;
