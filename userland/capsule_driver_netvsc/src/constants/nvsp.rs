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

//! The network VSP's own protocol on the channel (OpenVMM's netvsp_protocol
//! protocol.rs, cited "proto:LINE"; Linux's hyperv_net.h), each message a
//! u32 type and its fields, in band.

/// proto:38: the largest message, from version 6.1 on. Every message the
/// driver sends is this long, zero padded.
pub const MESSAGE: usize = 0x28;

/// proto:90-106, 132.
pub const INIT: u32 = 1;
pub const INIT_COMPLETE: u32 = 2;
pub const SEND_NDIS_VERSION: u32 = 100;
pub const SEND_RECEIVE_BUFFER: u32 = 101;
pub const SEND_RECEIVE_BUFFER_COMPLETE: u32 = 102;
pub const SEND_SEND_BUFFER: u32 = 104;
pub const SEND_SEND_BUFFER_COMPLETE: u32 = 105;
pub const SEND_RNDIS_PACKET: u32 = 107;
pub const SEND_RNDIS_PACKET_COMPLETE: u32 = 108;
pub const SEND_NDIS_CONFIG: u32 = 125;

/// proto:207-210.
pub const STATUS_SUCCESS: u32 = 1;

/// proto:40-41.
pub const DATA_CHANNEL: u32 = 0;
pub const CONTROL_CHANNEL: u32 = 1;

/// netvsp lib.rs:2400-2405 and Linux's ver_list, newest first: 6.1, 6, 5,
/// 4, 2, 1.
pub const VERSIONS: [u32; 6] = [0x60001, 0x60000, 0x50000, 0x40000, 0x30002, 0x2];
/// The version from which SEND_NDIS_CONFIG is sent (netvsp lib.rs:5134).
pub const V2: u32 = 0x30002;
/// NDIS 6.30 from protocol 5 on, 6.1 before (Linux's netvsc_connect_vsp).
pub const V5: u32 = 0x50000;
pub const NDIS_6_30: (u32, u32) = (6, 30);
pub const NDIS_6_1: (u32, u32) = (6, 1);

/// The receive buffer's id, Linux's NETVSC_RECEIVE_BUFFER_ID; the send
/// buffer's is 0 (hyperv_net.h:875-876).
pub const RECEIVE_BUFFER_ID: u16 = 0xCAFE;
pub const SEND_BUFFER_ID: u16 = 0;
/// An Ethernet frame without its FCS, the MTU netvsp takes by default and
/// at the least (netvsp lib.rs:3456-3457).
pub const MTU: u32 = 1514;
