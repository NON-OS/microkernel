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

//! One serial line each time a Linux program's window gets its surface,
//! whether a terminal or the store started it. Such a run's own output is
//! private and never reaches serial (debug.rs), so without this line a boot
//! log could not show that an app's window came. It names no app and holds
//! nothing the app drew: the size only, which every window on the screen
//! shows anyway.

use crate::kernel_core::surface_registry::SurfaceDescriptor;

pub(super) fn note(pid: u32, desc: &SurfaceDescriptor) {
    if !crate::userspace::capsule_linux::is_private_run(pid) {
        return;
    }
    let mut l = crate::sys::serial::Line::new();
    l.str(b"[LINUX] a window ").dec(u64::from(desc.width)).str(b"x");
    l.dec(u64::from(desc.height)).str(b" is shown").end();
}
