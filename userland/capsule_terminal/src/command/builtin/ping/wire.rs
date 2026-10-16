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

//! The net.ip service's wire, and how long an echo is waited for.

pub(super) const IP_SERVICE: &[u8] = b"net.ip";
pub(super) const IP_MAGIC: u32 = 0x4E49_5034;
pub(super) const HDR_LEN: usize = 20;
pub(super) const OP_SEND_PACKET: u16 = 4;
pub(super) const OP_POLL_PACKET: u16 = 5;
pub(super) const PROTO_ICMP: u8 = 1;
pub(super) const PING_ID: u16 = 0x4E4F;
pub(super) const DEADLINE_MS: i64 = 1000;
