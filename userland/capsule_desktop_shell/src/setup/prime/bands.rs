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

//! The two bands the shell's surfaces are placed in.

/// The desk's band: over the wallpaper's (0), under every application
/// window's (2).
pub const DESK_Z: u32 = 1;

/// The chrome's band: over every application window, under the boot splash
/// (4_000_000), which covers the whole screen while the system starts.
///
/// The shell used to hold one surface only, in the desk's band, so its menus,
/// the Launchpad, the right-click menu, toasts and consent dialogs all drew
/// under any window open where they opened, while their clicks were still
/// meant for them. The compositor keeps one layer per process and band, so
/// the shell holds both without anything new being granted to it.
pub const CHROME_Z: u32 = 3_000_000;
