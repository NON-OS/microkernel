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

mod backoff;
pub mod desktop;
mod dispatch;
pub mod dock_clock;
mod dock_sync;
pub mod grabs;
pub mod handlers;
mod input;
mod installed_apps;
mod launch_overdue;
mod packages;
mod paint_initial;
mod paste;
mod ready_to_block;
mod reap_tray;
mod refresh_taskbar;
mod repaint;
pub mod respond;
mod retry_input_subscription;
mod retry_wm_subscription;
pub mod runner;
pub mod wallpaper_policy;
mod store_changed;
mod store_health;
pub mod toast_clock;
mod wm_notify;
mod wm_notify_toast;

pub use runner::run;
