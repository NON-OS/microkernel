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

mod dock_full;
mod dock_reveal;
pub mod dock_rule;
mod dock_work;
pub mod expect;
mod expire_pulses;
mod expire_visibility;
mod focus_moved;
mod forget;
mod go_step;
mod mark_launch;
mod new;
mod reach;
mod route;
mod set_open;
mod types;
mod windows;

pub use dock_full::set_full_screen;
pub use dock_reveal::{dock_pointer, reveal_taskbar};
pub use dock_work::dock_work;
pub use expect::{expect_window, windows_overdue};
pub use expire_pulses::expire_taskbar_pulses;
pub use expire_visibility::expire_taskbar_visibility;
pub use focus_moved::focus_moved;
pub use forget::forget_dead_windows;
pub use go_step::{go_step, GoStep};
pub use mark_launch::mark_taskbar_launch;
pub use new::new_taskbar_state;
pub use reach::{reach_by, Reach};
pub use route::raise_tracked;
pub use types::{TaskbarState, Uptime, TASKBAR_NO_ACTIVE};
pub use windows::{track_window_closed, track_window_opened};
