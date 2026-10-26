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

mod apply_drain;
mod asks;
mod boot;
mod booted;
pub mod chrome;
mod click_focus;
mod control;
mod decorations;
mod dispatch;
mod drag;
mod ensure_primed;
#[cfg(feature = "runtime")]
mod entry;
#[cfg(feature = "runtime")]
mod ephemeral;
#[cfg(feature = "runtime")]
mod fail;
mod finish_band;
mod fit_display;
pub mod fit_rule;
mod frame_finish;
#[cfg(feature = "runtime")]
mod frame_loop;
pub mod full_screen_ask;
mod held;
mod idle;
pub mod inbox;
mod maximize;
pub mod min_size;
mod move_window;
pub mod no_window;
mod off_screen;
mod open_peers;
#[cfg(feature = "runtime")]
mod pace;
mod paint_draw;
mod paint_frame;
mod paint_once;
mod paint_partial;
mod press_part;
mod prime_frame;
pub mod rect_ask;
mod rect_follow;
mod refresh_input;
mod reopen;
mod repaint;
mod request_id;
mod resize_window;
mod restore;
mod run_loop;
mod service_frame;
mod teardown;
pub mod teardown_steps;
pub mod work_rect;

#[cfg(feature = "runtime")]
pub use entry::run;
pub use run_loop::run_loop;
