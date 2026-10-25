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

//! Each object kind's interface name, for a request this server does not
//! serve (unserved.rs).

use super::object::Object;

pub fn name(what: Object) -> &'static [u8] {
    match what {
        Object::Display => b"wl_display",
        Object::Registry => b"wl_registry",
        Object::Callback => b"wl_callback",
        Object::Compositor => b"wl_compositor",
        Object::Shm => b"wl_shm",
        Object::ShmPool => b"wl_shm_pool",
        Object::Buffer => b"wl_buffer",
        Object::Surface => b"wl_surface",
        Object::XdgBase => b"xdg_wm_base",
        Object::XdgSurface => b"xdg_surface",
        Object::XdgToplevel => b"xdg_toplevel",
        Object::Seat => b"wl_seat",
        Object::Pointer => b"wl_pointer",
        Object::Keyboard => b"wl_keyboard",
        Object::Output => b"wl_output",
        Object::Region => b"wl_region",
        Object::Subcompositor => b"wl_subcompositor",
        Object::Subsurface => b"wl_subsurface",
        Object::DataDeviceManager => b"wl_data_device_manager",
        Object::DataSource => b"wl_data_source",
        Object::DataDevice => b"wl_data_device",
        Object::DecorationManager => b"zxdg_decoration_manager_v1",
        Object::Decoration => b"zxdg_toplevel_decoration_v1",
        Object::Positioner => b"xdg_positioner",
        Object::Popup => b"xdg_popup",
        Object::Touch => b"wl_touch",
        Object::XdgOutputManager => b"zxdg_output_manager_v1",
        Object::XdgOutput => b"zxdg_output_v1",
        Object::Viewporter => b"wp_viewporter",
        Object::Viewport => b"wp_viewport",
        Object::Activation => b"xdg_activation_v1",
        Object::ActivationToken => b"xdg_activation_token_v1",
        Object::SinglePixel => b"wp_single_pixel_buffer_manager_v1",
        Object::PrimaryManager => b"zwp_primary_selection_device_manager_v1",
        Object::PrimaryDevice => b"zwp_primary_selection_device_v1",
        Object::PrimarySource => b"zwp_primary_selection_source_v1",
        Object::CursorManager => b"wp_cursor_shape_manager_v1",
        Object::CursorDevice => b"wp_cursor_shape_device_v1",
    }
}
