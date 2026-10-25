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

//! Whether a disk is the one this boot's block layer chose, pure: the
//! kernel's four-byte record (src/syscall/microkernel/install_source_in_use.rs)
//! names a backend and its instance, and that disk is never offered as an
//! install target (D13b). kernel_proofs::install_in_use.

/// Backend codes, as the kernel numbers them.
pub const VIRTIO_BLK: u8 = 1;
pub const AHCI: u8 = 2;
pub const NVME: u8 = 3;
pub const USB_MSC: u8 = 4;

/// The backend and instance a record names; None for a short record or
/// one naming no backend this knows, which withholds nothing.
pub fn parse_in_use(record: &[u8]) -> Option<(u8, u8)> {
    match record {
        [code @ (VIRTIO_BLK | AHCI | NVME | USB_MSC), instance, ..] => Some((*code, *instance)),
        _ => None,
    }
}

/// Whether the disk on backend `code`, instance `instance`, is in use.
pub fn in_use(chosen: Option<(u8, u8)>, code: u8, instance: u8) -> bool {
    chosen == Some((code, instance))
}
