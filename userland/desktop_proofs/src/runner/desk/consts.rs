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

//! The desk's bands, the dock's window and shadow, and what the chrome holds.

/// app_skeleton's layer band for application windows.
pub(super) const APP_LAYER_Z: u32 = 2;

/// The desktop shell's desk band, under every application window.
pub(super) const SHELL_LAYER_Z: u32 = 1;

/// The desktop shell's chrome band, over every application window
/// (capsule_desktop_shell setup/prime/register.rs CHROME_Z).
pub(super) const SHELL_CHROME_Z: u32 = 3_000_000;

/// The shell's dock window (state/chrome.rs TASKBAR_WINDOW_ID).
pub const DOCK_WINDOW_ID: u32 = 0x5442_4152;

/// The ring of shadow the shell draws round the dock at one to one
/// (render/panel.rs shadow_panel, px(3)).
pub(super) const DOCK_SHADOW: u32 = 3;

/// What the chrome surface holds at a pixel: nothing (transparent, the
/// windows under it show), the dock's panel, or its shadow.
pub(super) const CLEAR: u8 = 0;

pub(super) const PANEL: u8 = 1;

pub(super) const SHADOW: u8 = 2;
