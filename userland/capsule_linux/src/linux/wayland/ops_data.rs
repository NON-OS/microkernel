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

//! wl_data_device_manager, wl_data_source and wl_data_device, from
//! wayland.xml: requests, then the one event this server sends.

pub const MANAGER_CREATE_DATA_SOURCE: u16 = 0;
pub const MANAGER_GET_DATA_DEVICE: u16 = 1;
pub const SOURCE_OFFER: u16 = 0;
pub const SOURCE_DESTROY: u16 = 1;
pub const SOURCE_SET_ACTIONS: u16 = 2;
pub const DEVICE_START_DRAG: u16 = 0;
pub const DEVICE_SET_SELECTION: u16 = 1;
pub const DEVICE_RELEASE: u16 = 2;
pub const SOURCE_CANCELLED: u16 = 2;
