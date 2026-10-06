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







pub const LEG_HOST_FEATURES: usize = 0x00;
pub const LEG_GUEST_FEATURES: usize = 0x04;
pub const LEG_QUEUE_PFN: usize = 0x08;
pub const LEG_QUEUE_NUM: usize = 0x0C;
pub const LEG_QUEUE_SEL: usize = 0x0E;
pub const LEG_QUEUE_NOTIFY: usize = 0x10;
pub const LEG_STATUS: usize = 0x12;
pub const LEG_MAC: usize = 0x14;


pub const VIRTIO_NET_F_MAC: u32 = 5;
pub const VIRTIO_NET_F_STATUS: u32 = 16;


pub const VIRTIO_NET_S_LINK_UP: u16 = 1;
// Offset of the status word in struct virtio_net_config, after the MAC.
pub const NET_CFG_STATUS: usize = 6;

/*
 * The device-type features this driver takes. Never VIRTIO_NET_F_MAC: the
 * address a hypervisor or a card hands out names this machine to every
 * network it joins, and the virtio specification says a driver that does not
 * take the feature picks a random address, which `setup::station` does.
 */
pub const LEGACY_WANTED: u32 = 1 << VIRTIO_NET_F_STATUS;
pub const MODERN_WANTED: u64 = 1 << VIRTIO_NET_F_STATUS;
