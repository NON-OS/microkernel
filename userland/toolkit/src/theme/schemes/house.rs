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

//! The five schemes this desktop draws itself.

use super::Scheme;

/*
 * Each keeps the shape of the house palette: a ground with a colour cast rather
 * than pure black so raised surfaces have somewhere to rise from, a surface one
 * step up, text near white but not white, one accent, and a border that reads as a
 * line rather than an edge. Only the hue moves between them.
 */

/// The default, and the values `palette` has always compiled against.
pub const AURORA: Scheme = Scheme {
    bg: 0xFF0B_1319,
    surface: 0xFF13_1C24,
    accent: 0xFF35_C4E2,
    text: 0xFFE4_ECF5,
    border: 0xFF23_3243,
};

/// Neutral blue-grey for anyone who finds the cyan loud.
pub const SLATE: Scheme = Scheme {
    bg: 0xFF0F_172A,
    surface: 0xFF1E_293B,
    accent: 0xFF38_BDF8,
    text: 0xFFF1_F5F9,
    border: 0xFF33_4155,
};

/// No hue at all. The one scheme that stays legible on a display with a broken
/// colour channel, which is not hypothetical on second-hand hardware.
pub const MONO: Scheme = Scheme {
    bg: 0xFF12_1212,
    surface: 0xFF1E_1E1E,
    accent: 0xFFBD_BDBD,
    text: 0xFFED_EDED,
    border: 0xFF3A_3A3A,
};

pub const FOREST: Scheme = Scheme {
    bg: 0xFF0C_1512,
    surface: 0xFF15_2320,
    accent: 0xFF4F_C58B,
    text: 0xFFE6_F1EA,
    border: 0xFF27_3B35,
};

pub const SUNSET: Scheme = Scheme {
    bg: 0xFF19_0F12,
    surface: 0xFF26_181C,
    accent: 0xFFF2_8B54,
    text: 0xFFF6_E9E3,
    border: 0xFF3E_272C,
};
