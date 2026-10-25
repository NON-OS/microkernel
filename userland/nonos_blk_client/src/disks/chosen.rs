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

//! The disk this boot's block layer chose, asked of the kernel once per
//! scan (D13b).

use nonos_libc::install_source::{mk_install_source, INSTALL_SOURCE_IN_USE_DISK};

use super::in_use::{in_use, parse_in_use, AHCI, NVME, VIRTIO_BLK};
use crate::driver::Driver;

/// The backend and instance the kernel's block layer reads and writes; None
/// when it holds none yet, or the capsule may not ask.
pub(super) fn ask() -> Option<(u8, u8)> {
    let mut record = [0u8; 4];
    let n = mk_install_source(INSTALL_SOURCE_IN_USE_DISK, 0, &mut record);
    usize::try_from(n).ok().and_then(|n| parse_in_use(&record[..n.min(record.len())]))
}

/// Whether `driver`'s `instance` serves the disk this boot runs from.
pub(super) fn running_from(chosen: Option<(u8, u8)>, driver: Driver, instance: u8) -> bool {
    let code = match driver {
        Driver::VirtioBlk => VIRTIO_BLK,
        Driver::Ahci => AHCI,
        Driver::Nvme => NVME,
        // The kernel's block layer reads no SD card, and its record has no
        // code for a disk on the block wire: no record names either.
        Driver::Rtsx | Driver::Wire | Driver::Ufs | Driver::Ide | Driver::Scsi => return false,
    };
    in_use(chosen, code, instance)
}

/// Why a disk is listed but not offered: it may be the boot disk.
pub(super) const LOADER: &str =
    "holds the loader this boot ran, so it may be the boot disk: not offered";
/// Why a disk is listed but not offered: this boot runs from it (D13b).
pub(super) const RUNNING: &str = "this boot reads and writes its store now: not offered";
