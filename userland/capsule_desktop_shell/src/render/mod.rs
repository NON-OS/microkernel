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

pub mod attention;
mod blit;
mod blit_mean;
mod blit_over;
mod bottom_taskbar;
mod cap_names;
mod cap_table;
pub mod chrome;
pub mod consent;
mod delete_prompt;
pub mod desktop_icons;
pub mod desktop_menu;
mod dialog_kit;
mod dialog_text;
mod dock_marks;
pub mod fill;
mod icons;
mod input_menu;
pub mod launchpad;
pub mod layout;
pub mod live_prompt;
pub mod measure_aa;
pub mod menubar_menu;
mod off_tile;
pub mod palette;
pub mod panel;
pub mod pkg_consent;
mod power_off;
pub mod shadow_reach;
pub mod surface;
pub mod text_aa;
pub mod toasts;
pub mod topbar;
pub mod ui_font;
mod whole_frame;

pub use bottom_taskbar::paint_bottom_taskbar;
pub use chrome::paint_chrome;
pub use icons::{draw_app_icon, draw_tool_icon};
pub use layout::menubar_rect;
pub use toasts::sync_toast_layer;
