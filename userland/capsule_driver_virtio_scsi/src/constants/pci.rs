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
/// virtio-scsi, modern only: 0x1040 plus device type 8.
pub const VIRTIO_SCSI_MODERN_ID: u16 = 0x1048;
/// virtio-scsi, transitional: QEMU's default, which carries the modern
/// capabilities beside its legacy window. One without them is refused.
pub const VIRTIO_SCSI_TRANSITIONAL_ID: u16 = 0x1004;
