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

use super::Palette;

/// The light pages.
pub static LIGHT: Palette = Palette {
    ground: 0xFFF4_F7FC,
    surface: 0xFFFF_FFFF,
    sidebar: 0xFFE8_EEF8,
    field: 0xFFEE_F2F8,
    ink: 0xFF13_213A,
    ink2: 0xFF33_4460,
    muted: 0xFF5B_6B85,
    accent: 0xFF25_63EB,
    accent_soft: 0xFFDB_E7FF,
    sky: 0xFFA9_CBFA,
    shadow: 0x3010_1828,
};
