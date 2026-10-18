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

//! The record of the disk this boot's block layer chose, pure: the backend
//! and its instance, nothing of what is on the disk. An installer leaves
//! that disk off its list (D13b). The installer side reads it with
//! nonos_blk_client disks/in_use.rs. kernel_proofs::install_in_use.

/// The record's length.
pub(crate) const IN_USE_LEN: usize = 4;

/// Backend codes, shared with nonos_blk_client.
pub(crate) const VIRTIO_BLK: u8 = 1;
pub(crate) const AHCI: u8 = 2;
pub(crate) const NVME: u8 = 3;
pub(crate) const USB_MSC: u8 = 4;

/// The backend, its instance (the number in its `driver.<name>N` service),
/// and two reserved zero bytes.
pub(crate) const fn in_use_record(backend: u8, instance: u8) -> [u8; IN_USE_LEN] {
    [backend, instance, 0, 0]
}
