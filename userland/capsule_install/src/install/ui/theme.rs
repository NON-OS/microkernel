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

//! The installer's colours: the NØNOS palette (install/brand/palette.rs),
//! named for what each does on these screens.

use nonos_brand::palette as p;

pub const BACKGROUND: u32 = p::INK;
pub const HEADER_BG: u32 = p::INK;
pub const CARD_BG: u32 = p::CARD;
pub const CARD_BORDER: u32 = p::RULE;
pub const RULE: u32 = p::RULE;
pub const SELECTED_BG: u32 = p::CYAN_SOFT;

pub const TITLE: u32 = p::TEXT;
pub const FOREGROUND: u32 = p::TEXT;
pub const MUTED: u32 = p::TEXT_3;
pub const ACCENT: u32 = p::CYAN;
pub const WARN: u32 = p::WARN;
pub const DANGER: u32 = p::BAD;
pub const OK: u32 = p::CYAN;

pub const BAR_TRACK: u32 = p::RULE;
