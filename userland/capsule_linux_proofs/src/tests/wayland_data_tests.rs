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

//! The data device and decoration numbers, against wayland.xml and
//! xdg-decoration-unstable-v1.xml: a request is served by its number, and an
//! event read by its number, so a wrong one is another.

use crate::ops::{data, decoration};

#[test]
fn data_device_numbers_are_the_protocols() {
    assert_eq!((data::MANAGER_CREATE_DATA_SOURCE, data::MANAGER_GET_DATA_DEVICE), (0, 1));
    let source = (data::SOURCE_OFFER, data::SOURCE_DESTROY, data::SOURCE_SET_ACTIONS);
    assert_eq!(source, (0, 1, 2));
    let device = (data::DEVICE_START_DRAG, data::DEVICE_SET_SELECTION, data::DEVICE_RELEASE);
    assert_eq!(device, (0, 1, 2));
    // wl_data_source events: target, send, cancelled.
    assert_eq!(data::SOURCE_CANCELLED, 2);
}

#[test]
fn decoration_numbers_are_the_protocols() {
    use decoration::*;
    assert_eq!((MANAGER_DESTROY, MANAGER_GET_TOPLEVEL_DECORATION), (0, 1));
    let requests = (DECORATION_DESTROY, DECORATION_SET_MODE, DECORATION_UNSET_MODE);
    assert_eq!(requests, (0, 1, 2));
    assert_eq!(DECORATION_CONFIGURE, 0);
    // mode: client_side 1, server_side 2.
    assert_eq!(SERVER_SIDE, 2);
}
