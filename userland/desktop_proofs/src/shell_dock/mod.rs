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

//! The desktop shell's app table, instance names, dock and window notices,
//! under the `crate::state` and `crate::render` paths its files name.

// The desktop shell's app table, the one instance-name matcher and the dock's
// window tracking, under the `crate::state` paths the taskbar names them by.
#[path = "../../../capsule_desktop_shell/src/state/apps/mod.rs"]
pub mod shell_apps;

#[path = "../../../capsule_desktop_shell/src/state/instance/mod.rs"]
pub mod instance;

#[path = "../../../capsule_desktop_shell/src/state/taskbar/mod.rs"]
pub mod taskbar;

pub mod state {
    pub use crate::shell_apps as apps;
    pub use crate::shell_apps::DOCK_APPS;
    pub use crate::taskbar::Uptime;
}

// The window manager's lifecycle notification, as the shell reads it.
#[path = "../../../capsule_desktop_shell/src/state/wm_notice.rs"]
pub mod wm_notice;

// The shell's dock geometry: its panel, and the area a dock repaint commits.
#[path = "."]
pub mod render {
    pub use crate::shadow_reach;
    pub use crate::ui_font;
    #[path = "../../../capsule_desktop_shell/src/render/layout/mod.rs"]
    pub mod layout;
}
