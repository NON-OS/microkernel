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

//! The scene table, damage and how a frame is repainted from them.

mod attach_cache_tests;
mod attach_exit_tests;
mod attach_kernel;
mod attach_reap_tests;
mod attach_rig;
mod attach_tests;
mod band_remove_tests;
pub(crate) mod bound_model;
mod bound_session_tests;
mod bound_tests;
mod coalesce_tests;
mod damage_cover;
mod damage_full_tests;
mod damage_tests;
mod foreign_attach;
mod foreign_cursor_tests;
mod foreign_desk;
mod foreign_desk_assert;
mod foreign_desk_check;
mod foreign_desk_frame;
mod foreign_kernel;
mod foreign_pixel_tests;
mod foreign_shadow_tests;
mod frame_check;
pub(crate) mod frame_model;
pub(crate) mod frame_util;
mod heal_rig;
mod heal_tests;
mod indicator_tests;
mod raise_tests;
mod repaint_blank_tests;
mod repaint_frame;
mod repaint_raise_tests;
mod repaint_random_tests;
mod repaint_reap_tests;
mod repaint_rig;
mod repaint_tests;
mod stacking_band_tests;
mod stacking_click_tests;
mod stacking_rig;
mod stacking_tests;
