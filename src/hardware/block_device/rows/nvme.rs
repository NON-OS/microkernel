// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! NVMe (driver.nvme0). Its client maps sectors onto the namespace's LBAs;
//! only an LBA size it cannot map is passed over.

use core::sync::atomic::AtomicU64;

use super::super::disk::Disk;
use super::super::map_nvme::map_nvme_error;
use super::nvme_fit::nvme_sectors_fit;
use crate::hardware::nvme_capsule as c;

static SAID: AtomicU64 = AtomicU64::new(0);

pub(in super::super) static NVME: Disk = Disk {
    shown: |_| "NVMe (driver.nvme0)",
    announced: |_| "NVMe (driver.nvme0)",
    capacity: |_| c::capacity().map_err(map_nvme_error),
    read: |_, lba, out| c::read_blocks(lba, out).map_err(map_nvme_error),
    write: |_, lba, data| c::write_blocks(lba, data).map_err(map_nvme_error),
    flush: |_| c::flush().map_err(map_nvme_error),
    fits: |_| nvme_sectors_fit(),
    said: |_| &SAID,
    copy_answers: false,
    report: None,
};
