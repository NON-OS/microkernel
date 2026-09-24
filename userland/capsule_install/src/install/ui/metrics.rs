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

//! Sizes, in pixels. The window is dialog-shaped and sits where the
//! compositor's 1440x900 default puts a centred 760x540 panel; the frame
//! fits it to a smaller display.

pub const WIN_W: u32 = 760;
pub const WIN_H: u32 = 540;
pub const WIN_X: u32 = 340;
pub const WIN_Y: u32 = 160;

pub const HEADER_H: u32 = 64;
pub const FOOTER_H: u32 = 44;
pub const PAD: u32 = 28;

pub const TITLE_PX: f32 = 22.0;
pub const BODY_PX: f32 = 17.0;
pub const SMALL_PX: f32 = 14.0;
pub const MONO_PX: f32 = 16.0;

pub const LINE_H: u32 = 26;
pub const ROW_H: u32 = 56;
pub const BAR_H: u32 = 14;
pub const RADIUS: u32 = 9;
