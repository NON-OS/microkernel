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

pub fn tile_color(v: u64) -> u32 {
    match v {
        0 => 0xFF2A2A3E,
        2 => 0xFFEEE4DA,
        4 => 0xFFEDE0C8,
        8 => 0xFFF2B179,
        16 => 0xFFF59563,
        32 => 0xFFF67C5F,
        64 => 0xFFF65E3B,
        128 => 0xFFEDCF72,
        256 => 0xFFEDCC61,
        512 => 0xFFEDC850,
        1024 => 0xFFEDC53F,
        _ => 0xFFEDC22E,
    }
}

/// A tile's digits: the game's dark ground on every tile, all of which are
/// light, so each value reads at WCAG AA, 4.5:1 or more (demo_screen_proofs
/// game_2048_palette.rs). White on the 2 tile was 1.2:1.
pub fn text_color(_v: u64) -> u32 {
    INK
}

/// The ground the board sits on, and the ink of every tile's digits.
pub const INK: u32 = 0xFF1A1A2E;
