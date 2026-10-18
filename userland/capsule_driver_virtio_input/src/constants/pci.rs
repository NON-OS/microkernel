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
/// virtio-input has no transitional id: 0x1040 plus device type 18.
pub const VIRTIO_INPUT_DEVICE_ID: u16 = 0x1052;
/// QEMU gives the tablet, the keyboard and the mouse a function each; one
/// capsule drives every one it finds, up to this many.
pub const MAX_INPUTS: usize = 4;
