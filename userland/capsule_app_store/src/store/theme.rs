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
// The house palette, the same values the About and Settings restyles
// established. Layout sizes are not here: they live in ui/metrics.rs.
pub const BACKGROUND: u32 = 0xFF0B1319;
pub const CARD_BG: u32 = 0xFF0E1920;
pub const CARD_SEL_BG: u32 = 0xFF13242E;
pub const CARD_SEL_EDGE: u32 = 0xFF35C4E2;
pub const PANE_BG: u32 = 0xFF101C24;
pub const STATUS_BG: u32 = 0xFF0D171E;
pub const RULE: u32 = 0xFF16262F;

pub const TITLE: u32 = 0xFFEAF4F8;
pub const FOREGROUND: u32 = 0xFFCDDDE5;
pub const MUTED: u32 = 0xFF6D818C;
pub const ACCENT: u32 = 0xFF35C4E2;

pub const TAB_FG: u32 = 0xFF93A7B2;
pub const TAB_FG_ACTIVE: u32 = 0xFFA8E7F6;
pub const TAB_BG_ACTIVE: u32 = 0x2035C4E2;

/// The tile behind a listing's initial. Tinted per source so the three
/// namespaces are distinguishable before a single word is read.
pub const TILE_NONOS: u32 = 0xFF17323D;
pub const TILE_LINUX: u32 = 0xFF1B2E3A;
pub const TILE_COMMUNITY: u32 = 0xFF2A2438;

/// The install action, filled rather than lettered: a coloured word is not
/// obviously a control, and every row on this screen is an offer to do
/// something.
pub const BUTTON_BG: u32 = 0xFF1B6E5A;
pub const BUTTON_FG: u32 = 0xFFD6F5EA;
pub const BUTTON_OFF_BG: u32 = 0xFF17242B;
pub const BUTTON_OFF_FG: u32 = 0xFF6D818C;

pub const OK: u32 = 0xFF33CF7D;
pub const DANGER: u32 = 0xFFE06C75;
