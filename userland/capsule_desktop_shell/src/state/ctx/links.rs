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

//! Where the shell stands with its peers: the wallpaper told the scaling,
//! the router's input subscription and grab, and the window manager's notices.

pub struct Links {
    /// Whether the wallpaper has taken the desktop's scaling yet
    /// (server/wallpaper_policy.rs).
    pub wallpaper_policy_sent: bool,
    pub input_kind_mask: u32,
    pub input_ready: bool,
    pub wm_notify_ready: bool,
    /// The router grab mask the shell holds (state/grab_rule.rs).
    pub grab_held: u32,
}
