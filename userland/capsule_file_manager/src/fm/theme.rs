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

// Surfaces, from the app ground up to a raised/hovered fill.
pub const VOID: u32 = 0xFF03070E;
pub const DEEP: u32 = 0xFF061019;
pub const WIN: u32 = 0xFF08131F;
pub const PANEL: u32 = 0xFF0B1A28;
pub const RAISE: u32 = 0xFF0E2233;

// Borders: a hairline for separators, an emphasised one for focused edges.
pub const LINE: u32 = 0xFF12303F;
pub const LINE2: u32 = 0xFF1B4257;

// Accent.
pub const CY: u32 = 0xFF22D3EE;
pub const CY_DIM: u32 = 0xFF1BA8BE;

// Text, primary through tertiary/label.
pub const INK: u32 = 0xFFDCEDF5;
pub const INK2: u32 = 0xFF96B2C2;
pub const INK3: u32 = 0xFF6A8798;

// Filetype and status hues.
pub const BLUE: u32 = 0xFF2F86D8;
pub const BLUE_LT: u32 = 0xFF5CC0F5;
pub const RED: u32 = 0xFFF0665A;
pub const AMBER: u32 = 0xFFE8B457;
pub const GRN: u32 = 0xFF5FC26B;

// Names the current painters still use, mapped onto the token set above so the
// palette moves in one place while each surface is rebuilt.
pub const BACKGROUND: u32 = WIN;
pub const FOREGROUND: u32 = INK;
pub const SELECTED: u32 = CY;
pub const DIRECTORY: u32 = CY_DIM;
pub const MUTED: u32 = INK3;
pub const HEADER_BG: u32 = DEEP;
pub const ALT_ROW: u32 = PANEL;
pub const SELECT_BG: u32 = RAISE;
pub const ACCENT: u32 = CY;
pub const FILE_C: u32 = INK2;
