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

//! The ports of the services the shell talks to, as the registry gave them
//! at start (the wallpaper's and the policy store's 0 until they are found).

pub struct Ports {
    pub compositor_port: u32,
    pub wm_port: u32,
    pub input_router_port: u32,
    /// The wallpaper service's port, 0 until it registers.
    pub wallpaper_port: u32,
    pub policy_port: u32,
}
