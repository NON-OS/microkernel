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

//! The globals a client is told about, and what binding one means.

use super::global::{g, Global};
use super::object::Object;

pub const GLOBALS: &[Global] = &[
    g(1, b"wl_compositor", 6, Object::Compositor),
    g(2, b"wl_shm", 1, Object::Shm),
    g(3, b"xdg_wm_base", 2, Object::XdgBase),
    g(4, b"wl_seat", 5, Object::Seat),
    g(5, b"wl_output", 4, Object::Output),
    g(6, b"wl_subcompositor", 1, Object::Subcompositor),
    g(7, b"wl_data_device_manager", 3, Object::DataDeviceManager),
    g(8, b"zxdg_decoration_manager_v1", 1, Object::DecorationManager),
    g(9, b"zxdg_output_manager_v1", 2, Object::XdgOutputManager),
    g(10, b"wp_viewporter", 1, Object::Viewporter),
    g(11, b"xdg_activation_v1", 1, Object::Activation),
    g(12, b"wp_single_pixel_buffer_manager_v1", 1, Object::SinglePixel),
    g(13, b"zwp_primary_selection_device_manager_v1", 1, Object::PrimaryManager),
    g(14, b"wp_cursor_shape_manager_v1", 1, Object::CursorManager),
    g(15, b"wp_fractional_scale_manager_v1", 1, Object::FractionalManager),
    g(16, b"org_kde_kwin_server_decoration_manager", 1, Object::KdeDecorationManager),
];
