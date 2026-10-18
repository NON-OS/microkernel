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
// Offsets in struct virtio_gpu_config, inside the device configuration
// region the capability places. These used to be the legacy window's own
// offsets, which on a modern device read past the 16-byte structure (QEMU
// answers all ones there) and reported 0xFFFFFFFF scanouts.
pub const GPU_CFG_EVENTS_READ: usize = 0x00;
pub const GPU_CFG_EVENTS_CLEAR: usize = 0x04;
/// The events word's display bit: the host's displays changed.
pub const GPU_EVENT_DISPLAY: u32 = 1 << 0;
pub const VG_MAX_SCANOUTS: usize = 16;
pub const VG_FORMAT_B8G8R8A8_UNORM: u32 = 1;
