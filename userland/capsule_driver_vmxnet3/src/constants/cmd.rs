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

//! Commands written to CMD; a GET command's answer is read back from CMD.

pub const ACTIVATE_DEV: u32 = 0xCAFE_0000;
pub const QUIESCE_DEV: u32 = 0xCAFE_0001;
pub const RESET_DEV: u32 = 0xCAFE_0002;
pub const UPDATE_RX_MODE: u32 = 0xCAFE_0003;
pub const UPDATE_MAC_FILTERS: u32 = 0xCAFE_0004;
pub const UPDATE_VLAN_FILTERS: u32 = 0xCAFE_0005;
pub const GET_QUEUE_STATUS: u32 = 0xF00D_0000;
pub const GET_LINK: u32 = 0xF00D_0002;

/// GET_LINK: bit 0 is the link, the upper half the speed.
pub const LINK_UP: u32 = 1;
/// Unicast to the station address and broadcast; no multicast is joined.
pub const RXM_UCAST: u32 = 0x01;
pub const RXM_BCAST: u32 = 0x04;
/// The event cause bits for a queue the device stopped on an error.
pub const ECR_QUEUE_ERRORS: u32 = 0x03;
