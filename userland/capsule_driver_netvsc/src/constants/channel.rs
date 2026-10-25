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

//! The channel's record, as the VMBus core has the kernel make it: vendor
//! 0x1414 (Microsoft) and the network interface, its relid in `irq_source`.

pub const VMBUS_VENDOR: u16 = 0x1414;
/// The synthetic network interface (f8615163-df3e-46c5-913f-f2d2f965ed0e).
pub const NETWORK_DEVICE: u16 = 0x4E45;
/// The open's user data: none, as Linux's netvsc_open_channel hands none.
pub const OPEN_USER: [u8; 0] = [];
/// The three GPADLs: "NE", which buffer, and the relid, so no two channels'
/// share an id.
pub const fn gpadl_rings(relid: u32) -> u32 {
    0x4E45_0000 | (relid & 0x0FFF)
}
pub const fn gpadl_recv(relid: u32) -> u32 {
    0x4E45_1000 | (relid & 0x0FFF)
}
pub const fn gpadl_send(relid: u32) -> u32 {
    0x4E45_2000 | (relid & 0x0FFF)
}
