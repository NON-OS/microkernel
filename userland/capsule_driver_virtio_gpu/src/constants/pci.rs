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
pub const VIRTIO_VENDOR_ID: u16 = 0x1AF4;
/// virtio-gpu is a virtio 1.0 device: the specification gives it no
/// transitional id, and QEMU builds virtio-vga and virtio-gpu-pci modern
/// whatever disable-modern says. Nothing presents 0x1010.
pub const VIRTIO_GPU_MODERN: u16 = 0x1050;
