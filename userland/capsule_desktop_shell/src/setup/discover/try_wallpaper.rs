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

/// The wallpaper service's port, or 0 while it is not registered. The desktop
/// does not wait for it: the wallpaper only paints under the shell, and a slow
/// one (its catalog reading from the store) once held the whole desktop back
/// in "setup stuck: wallpaper call failed". The runner asks again.
pub fn try_wallpaper() -> u32 {
    super::lookup_port::lookup_port(super::constants::WALLPAPER_SERVICE).unwrap_or(0)
}
