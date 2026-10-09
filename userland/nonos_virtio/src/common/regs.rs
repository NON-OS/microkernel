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

//! struct virtio_pci_common_cfg, virtio 1.0 section 4.1.4.3.

pub const DEVICE_FEATURE_SELECT: usize = 0x00;
pub const DEVICE_FEATURE: usize = 0x04;
pub const DRIVER_FEATURE_SELECT: usize = 0x08;
pub const DRIVER_FEATURE: usize = 0x0C;
pub const MSIX_CONFIG: usize = 0x10;
pub const NUM_QUEUES: usize = 0x12;
pub const DEVICE_STATUS: usize = 0x14;
pub const CONFIG_GENERATION: usize = 0x15;
pub const QUEUE_SELECT: usize = 0x16;
pub const QUEUE_SIZE: usize = 0x18;
pub const QUEUE_MSIX_VECTOR: usize = 0x1A;
pub const QUEUE_ENABLE: usize = 0x1C;
pub const QUEUE_NOTIFY_OFF: usize = 0x1E;
pub const QUEUE_DESC: usize = 0x20;
pub const QUEUE_DRIVER: usize = 0x28;
pub const QUEUE_DEVICE: usize = 0x30;
