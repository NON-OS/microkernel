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

pub mod apps;
pub mod chrome;
pub mod context;
pub mod delete_prompt;
pub mod dialog_keys;
pub mod grab_rule;
pub mod indicators;
pub mod instance;
pub mod launch;
pub mod launchpad_wheel;
pub mod live_prompt;
pub mod menubar;
pub mod notify;
pub mod open_arg;
pub mod paste_line;
pub mod pkg_prompt;
pub mod quiet_gap;
pub mod says;
pub mod scale;
pub mod store_word;
pub mod system_key;
pub mod taskbar;
pub mod toast;
pub mod toasts;
pub mod tool_apps;
pub mod tray;
pub mod volume;
pub mod wm_notice;

pub use apps::{DOCK_APPS, LAUNCHER_APPS};
pub use chrome::{TASKBAR_WINDOW_ID, TOAST_WINDOW_ID};
pub use context::Context;
pub use menubar::{new_menubar_state, MenubarState};
pub use notify::NotifyLevel;
pub use pkg_prompt::PkgInstallPrompt;
pub use taskbar::{
    dock_pointer, dock_work, expect_window, expire_taskbar_pulses, expire_taskbar_visibility,
    forget_dead_windows, go_step, mark_taskbar_launch, new_taskbar_state, raise_tracked, reach_by,
    reveal_taskbar, set_full_screen, track_window_closed, track_window_opened, windows_overdue,
    GoStep, Reach, TaskbarState, Uptime, TASKBAR_NO_ACTIVE,
};
pub use toasts::ToastQueue;
pub use tool_apps::TOOL_APPS;
pub use tray::{TrayEntry, TrayTable};
