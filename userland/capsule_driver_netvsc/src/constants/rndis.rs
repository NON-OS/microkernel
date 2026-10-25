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

//! RNDIS inside the network channel (OpenVMM's netvsp_protocol rndisprot.rs,
//! cited "rndis:LINE"; Linux's rndis.h). Every message starts with its type
//! and its whole length; offsets inside count from the field after them.

/// rndis:221-237.
pub const PACKET_MSG: u32 = 0x0000_0001;
pub const INITIALIZE_MSG: u32 = 0x0000_0002;
pub const QUERY_MSG: u32 = 0x0000_0004;
pub const SET_MSG: u32 = 0x0000_0005;
pub const INDICATE_STATUS_MSG: u32 = 0x0000_0007;
pub const INITIALIZE_CMPLT: u32 = 0x8000_0002;
pub const QUERY_CMPLT: u32 = 0x8000_0004;
pub const SET_CMPLT: u32 = 0x8000_0005;

/// rndis:30, 44-45.
pub const STATUS_SUCCESS: u32 = 0;
pub const STATUS_MEDIA_CONNECT: u32 = 0x4001_000B;
pub const STATUS_MEDIA_DISCONNECT: u32 = 0x4001_000C;

/// Linux's rndis.h:227, 235, 323.
pub const OID_GEN_CURRENT_PACKET_FILTER: u32 = 0x0001_010E;
pub const OID_GEN_MEDIA_CONNECT_STATUS: u32 = 0x0001_0114;
pub const OID_GEN_RNDIS_CONFIG_PARAMETER: u32 = 0x0001_021B;
pub const OID_802_3_PERMANENT_ADDRESS: u32 = 0x0101_0101;
/// NdisMediaStateConnected.
pub const MEDIA_CONNECTED: u32 = 0;

/// rndis:1128-1131: frames to the station, every multicast and broadcast.
pub const FILTER: u32 = 0x1 | 0x4 | 0x8;

/// RNDIS 1.0, and Linux's largest transfer.
pub const MAJOR: u32 = 1;
pub const MINOR: u32 = 0;
pub const MAX_TRANSFER: u32 = 0x4000;

/// The data message's header (rndis:507): its 8-byte message header, then
/// nine u32 fields; data_offset counts from the first of them.
pub const PACKET_HEADER: usize = 8 + 36;

/// Linux's RNDIS_CONFIG_PARAM_TYPE_STRING and "NetworkAddress".
pub const PARAM_STRING: u32 = 2;
pub const NETWORK_ADDRESS: &str = "NetworkAddress";
