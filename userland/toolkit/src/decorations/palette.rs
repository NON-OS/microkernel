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
//! The frame's colours. All but the shadow and the close control's hover are
//! one step from a live theme role (theme/chrome), so a window follows Ink,
//! Paper and High contrast with the app inside it.

use crate::theme::chrome;
use crate::theme::palette::ERROR;

pub const SHADOW: u32 = 0xA000_0508;
pub const TRANSPARENT: u32 = 0x0000_0000;
/// A hovered close control: the error colour, so the one control that loses
/// work says so before it is pressed.
pub const CLOSE_HOVER: u32 = ERROR;
pub const ON_CLOSE_HOVER: u32 = 0xFFFF_FFFF;

/// The frame behind the title bar and under the app: the window's own
/// ground, so the bar is part of the window and the rule under it divides.
pub fn frame_bg() -> u32 {
    chrome::window_bg()
}

/// The frame's edge and the rule under the title bar.
pub fn frame_edge() -> u32 {
    chrome::hairline()
}

/// The title, dimmed on a window that does not have the keyboard.
pub fn title_ink(focused: bool) -> u32 {
    if focused {
        chrome::ink()
    } else {
        chrome::muted()
    }
}

/// A control's glyph at rest: no colour, so meaning never rests on colour.
pub fn glyph_rest() -> u32 {
    chrome::muted()
}

pub fn glyph_hover() -> u32 {
    chrome::ink()
}

pub fn control_wash() -> u32 {
    chrome::hover_wash()
}

/// The 1 px rule along the top of the focused window: where the keys go.
pub fn focus_rule() -> u32 {
    chrome::accent()
}
