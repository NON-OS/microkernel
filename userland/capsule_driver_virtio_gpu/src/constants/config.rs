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
// Offsets in struct virtio_gpu_config. The register type adds where that
// structure starts: LEG_DEVICE_CFG in the legacy window, the device
// capability's place on the modern transport. These used to be the legacy
// window's own offsets, which on a modern device read past the 16-byte
// structure (QEMU answers all ones there) and reported 0xFFFFFFFF scanouts.
pub const GPU_CFG_EVENTS_READ: usize = 0x00;
pub const GPU_CFG_NUM_SCANOUTS: usize = 0x08;
pub const GPU_CFG_NUM_CAPSETS: usize = 0x0C;
pub const VG_MAX_SCANOUTS: usize = 16;
pub const VG_FORMAT_B8G8R8A8_UNORM: u32 = 1;
