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

//! The legacy virtio PCI registers, in the device's I/O BAR (recalled).

pub(super) const STATUS_ACKNOWLEDGE: u8 = 1;
pub(super) const STATUS_DRIVER: u8 = 2;
pub(super) const STATUS_DRIVER_OK: u8 = 4;

pub(super) const HOST_FEATURES: u16 = 0x00;
pub(super) const GUEST_FEATURES: u16 = 0x04;
pub(super) const QUEUE_PFN: u16 = 0x08;
pub(super) const QUEUE_NUM: u16 = 0x0C;
pub(super) const QUEUE_SEL: u16 = 0x0E;
pub(super) const QUEUE_NOTIFY: u16 = 0x10;
pub(super) const STATUS: u16 = 0x12;
/// Reading it clears the device's interrupt line.
pub(super) const ISR: u16 = 0x13;
