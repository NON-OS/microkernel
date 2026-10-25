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

//! The four profiles. Meaning colours come from the system's canonical eight
//! on the dark grounds. The light one is macOS Terminal's Basic, near black on
//! pure white, and darkens the same hues, as a pale accent on white carries no signal.

use super::types::Theme;

#[path = "profiles_pick.rs"]
mod profiles_pick;
pub use profiles_pick::{by_index, by_name};

pub const DARK: Theme = Theme {
    bg: 0xFF07_090B,
    fg: 0xFFD6_DCE3,
    accent: 0xFF5A_E6D0,
    path: 0xFF7C_F0A8,
    dim: 0xFF7A_828D,
    ok: 0xFF35_D07A,
    err: 0xFFE6_494E,
    run: 0xFF5F_6772,
    chrome_edge: 0xFF1A_1F24,
};

pub const DIM: Theme = Theme {
    bg: 0xB00B_0E12,
    fg: 0xFFE2_E8EF,
    accent: 0xFF5A_E6D0,
    path: 0xFF7C_F0A8,
    dim: 0xFF7D_8690,
    ok: 0xFF35_D07A,
    err: 0xFFEA_4D52,
    run: 0xFF7D_8690,
    chrome_edge: 0xFF23_2A31,
};

pub const LIGHT: Theme = Theme {
    bg: 0xFFFF_FFFF,
    fg: 0xFF1D_1D1F,
    accent: 0xFF0E_7A70,
    path: 0xFF15_694A,
    dim: 0xFF65_6D77,
    ok: 0xFF12_784A,
    err: 0xFFB0_1218,
    run: 0xFF83_8992,
    chrome_edge: 0xFFD9_D9D9,
};

pub const ABYSS: Theme = Theme {
    bg: 0xFF00_0308,
    fg: 0xFFC8_D2DC,
    accent: 0xFF5A_E6D0,
    path: 0xFF7C_F0A8,
    dim: 0xFF77_7F88,
    ok: 0xFF35_D07A,
    err: 0xFFE5_484D,
    run: 0xFF5B_636C,
    chrome_edge: 0xFF0A_0E13,
};

pub const COUNT: u16 = 4;
