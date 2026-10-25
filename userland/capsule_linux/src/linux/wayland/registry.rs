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

use super::global::Global;
use super::object::Object;

pub const GLOBALS: &[Global] = &[
    Global { name: 1, interface: b"wl_compositor", version: 4, object: Object::Compositor },
    Global { name: 2, interface: b"wl_shm", version: 1, object: Object::Shm },
    Global { name: 3, interface: b"xdg_wm_base", version: 2, object: Object::XdgBase },
    Global { name: 4, interface: b"wl_seat", version: 5, object: Object::Seat },
    Global { name: 5, interface: b"wl_output", version: 2, object: Object::Output },
    Global { name: 6, interface: b"wl_subcompositor", version: 1, object: Object::Subcompositor },
    Global {
        name: 7,
        interface: b"wl_data_device_manager",
        version: 3,
        object: Object::DataDeviceManager,
    },
    Global {
        name: 8,
        interface: b"zxdg_decoration_manager_v1",
        version: 1,
        object: Object::DecorationManager,
    },
    Global {
        name: 9,
        interface: b"zxdg_output_manager_v1",
        version: 2,
        object: Object::XdgOutputManager,
    },
    Global { name: 10, interface: b"wp_viewporter", version: 1, object: Object::Viewporter },
    Global { name: 11, interface: b"xdg_activation_v1", version: 1, object: Object::Activation },
    Global {
        name: 12,
        interface: b"wp_single_pixel_buffer_manager_v1",
        version: 1,
        object: Object::SinglePixel,
    },
    Global {
        name: 13,
        interface: b"zwp_primary_selection_device_manager_v1",
        version: 1,
        object: Object::PrimaryManager,
    },
    Global {
        name: 14,
        interface: b"wp_cursor_shape_manager_v1",
        version: 1,
        object: Object::CursorManager,
    },
];
