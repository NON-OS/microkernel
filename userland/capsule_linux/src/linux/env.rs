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

//! The environment the first program of a guest starts with.

use alloc::vec::Vec;

/// Deliberately short.
pub fn default() -> Vec<Vec<u8>> {
    alloc::vec![
        b"PATH=/usr/local/bin:/usr/bin:/bin:/usr/local/sbin:/usr/sbin:/sbin".to_vec(),
        b"HOME=/root".to_vec(),
        b"TERM=linux".to_vec(),
        b"PWD=/".to_vec(),
        b"SHELL=/bin/sh".to_vec(),
        b"LANG=C.UTF-8".to_vec(),
        // libwayland-client will not look for a display without a runtime
        // directory; /run is private to each guest, and any path ending in
        // wayland-0 reaches the personality's compositor (unix/path.rs).
        b"XDG_RUNTIME_DIR=/run/user/0".to_vec(),
        b"WAYLAND_DISPLAY=wayland-0".to_vec(),
        // What gtk_prep.rs makes in this run's private /tmp, for GTK.
        b"GSETTINGS_SCHEMA_DIR=/tmp".to_vec(),
        b"GDK_PIXBUF_MODULE_FILE=/tmp/loaders.cache".to_vec(),
        b"XDG_DATA_DIRS=/tmp/share:/usr/local/share:/usr/share".to_vec(),
        // A window program's toolkit is told it is on Wayland, and drawn
        // without a GPU, which no guest has: GTK 4 through cairo, Qt Quick in
        // software. Qt and GDK are held to Wayland, as there is no X server
        // to fall back to, and GTK's accessibility bridge is not started,
        // as there is no session bus for it.
        b"XDG_SESSION_TYPE=wayland".to_vec(),
        b"GDK_BACKEND=wayland".to_vec(),
        b"QT_QPA_PLATFORM=wayland".to_vec(),
        b"GSK_RENDERER=cairo".to_vec(),
        b"QT_QUICK_BACKEND=software".to_vec(),
        b"NO_AT_BRIDGE=1".to_vec(),
        b"EGL_PLATFORM=wayland".to_vec(),
    ]
}

/// Where a box's closure places the NØNOS-built Mesa.
pub const MESA: &[u8] = b"/usr/lib/nonos-mesa";

/// The same, with PWD the directory the first program starts in. A program
/// that asks EGL for OpenGL is sent to Mesa's software driver, drawn into
/// wl_shm, only where the run's store has it (`mesa`): pointed at a driver
/// that is not there, every Qt program said Mesa could not open it.
pub fn with_pwd(cwd: &[u8], mesa: bool) -> Vec<Vec<u8>> {
    let mut env = default();
    env.iter_mut().filter(|v| v.starts_with(b"PWD=")).for_each(|v| *v = [b"PWD=", cwd].concat());
    if mesa {
        env.push(b"LIBGL_ALWAYS_SOFTWARE=1".to_vec());
        env.push([b"LIBGL_DRIVERS_PATH=", MESA].concat());
    } else {
        // No driver to open: EGL's loader and Qt's Wayland layer each said
        // so, a dozen lines, before every Qt window drew into wl_shm.
        env.push(b"EGL_LOG_LEVEL=fatal".to_vec());
        env.push(b"QT_LOGGING_RULES=qt.qpa.wayland.warning=false".to_vec());
    }
    env
}
