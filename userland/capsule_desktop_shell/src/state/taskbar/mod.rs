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

pub mod dock_rule;
pub mod expect;
mod expire_pulses;
mod expire_visibility;
mod go_step;
mod mark_launch;
mod new;
mod route;
mod set_open;
mod types;
mod windows;

pub use dock_rule::{dock_pointer, dock_work, reveal_taskbar, set_full_screen};
pub use expect::{expect_window, windows_overdue};
pub use expire_pulses::expire_taskbar_pulses;
pub use expire_visibility::expire_taskbar_visibility;
pub use go_step::{go_step, GoStep};
pub use mark_launch::mark_taskbar_launch;
pub use new::new_taskbar_state;
pub use route::{raise_tracked, reach_by, Reach};
pub use types::{TaskbarState, Uptime, TASKBAR_NO_ACTIVE};
pub use windows::{forget_dead_windows, track_window_closed, track_window_opened};
