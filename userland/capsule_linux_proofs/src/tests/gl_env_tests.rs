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

//! OpenGL in a box is Mesa's software driver, found where the box's closure
//! places it and drawn through Wayland (docs/design/box-gl.md, G.1).

use crate::linux::env::{default, with_pwd};

const GL: [&[u8]; 3] = [
    b"LIBGL_ALWAYS_SOFTWARE=1",
    b"LIBGL_DRIVERS_PATH=/usr/lib/nonos-mesa",
    b"EGL_PLATFORM=wayland",
];

#[test]
fn every_guest_is_told_to_draw_gl_in_software_through_wayland() {
    for env in [default(), with_pwd(b"/root")] {
        for want in GL {
            assert_eq!(env.iter().filter(|v| v.as_slice() == want).count(), 1, "{want:?}");
        }
    }
}

#[test]
fn no_guest_is_pointed_at_a_gpu_driver() {
    let env = default();
    let named = |key: &[u8]| env.iter().any(|v| v.starts_with(key));
    assert!(!named(b"GALLIUM_DRIVER="), "softpipe for all would pin out llvmpipe");
    assert!(!named(b"MESA_LOADER_DRIVER_OVERRIDE="));
}
