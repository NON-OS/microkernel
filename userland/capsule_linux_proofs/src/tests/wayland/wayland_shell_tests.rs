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

//! The xdg-shell numbers the server takes, against xdg-shell.xml
//! and wayland.xml, laid out in each interface's own order so a gap or a
//! swap between the two tables shows.

use crate::ops::{ev, req, shell, ERROR_INVALID_OBJECT};

#[test]
fn xdg_toplevel_requests_run_zero_to_thirteen() {
    let toplevel = [
        req::TOPLEVEL_DESTROY,
        shell::TOPLEVEL_SET_PARENT,
        req::TOPLEVEL_SET_TITLE,
        req::TOPLEVEL_SET_APP_ID,
        shell::TOPLEVEL_SHOW_WINDOW_MENU,
        shell::TOPLEVEL_MOVE,
        shell::TOPLEVEL_RESIZE,
        shell::TOPLEVEL_SET_MAX_SIZE,
        shell::TOPLEVEL_SET_MIN_SIZE,
        req::TOPLEVEL_SET_MAXIMIZED,
        req::TOPLEVEL_UNSET_MAXIMIZED,
        req::TOPLEVEL_SET_FULLSCREEN,
        req::TOPLEVEL_UNSET_FULLSCREEN,
        shell::TOPLEVEL_SET_MINIMIZED,
    ];
    assert!(toplevel.iter().copied().eq(0..14));
}

#[test]
fn wm_base_surface_positioner_and_popup() {
    let base = [
        shell::BASE_DESTROY,
        shell::BASE_CREATE_POSITIONER,
        req::XDG_BASE_GET_SURFACE,
        req::XDG_BASE_PONG,
    ];
    assert!(base.iter().copied().eq(0..4));
    let surface = [
        req::XDG_SURFACE_DESTROY,
        req::XDG_SURFACE_GET_TOPLEVEL,
        shell::SURFACE_GET_POPUP,
        req::XDG_SURFACE_SET_GEOMETRY,
        req::XDG_SURFACE_ACK_CONFIGURE,
    ];
    assert!(surface.iter().copied().eq(0..5));
    // destroy, then set_size .. set_parent_configure.
    assert_eq!(
        (shell::POSITIONER_DESTROY, shell::POSITIONER_SET_FIRST, shell::POSITIONER_SET_LAST),
        (0, 1, 9)
    );
    assert_eq!((shell::POPUP_DESTROY, shell::POPUP_GRAB), (0, 1));
    // xdg_popup events: configure 0, popup_done 1.
    assert_eq!(shell::POPUP_DONE, 1);
}

#[test]
fn display_error_is_event_zero_with_invalid_object_zero() {
    // wl_display events: error 0, delete_id 1; error codes: invalid_object 0.
    assert_eq!((ev::DISPLAY_ERROR, ev::DISPLAY_DELETE_ID, ERROR_INVALID_OBJECT), (0, 1, 0));
}
